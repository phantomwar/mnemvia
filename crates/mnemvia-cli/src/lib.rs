use std::{
    collections::HashSet,
    fs,
    path::{Component, Path},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

pub const SCHEMA_VERSION: &str = "4";
pub const TOKENIZER_ID: &str = "heuristic-v1";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct IngestSummary {
    pub operation_id: i64,
    pub discovered: usize,
    pub inserted: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub revoked: usize,
    pub suppressed: usize,
    pub skipped: usize,
    pub facts: usize,
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub source_id: i64,
    pub path: String,
    pub score: f64,
    pub snippet: String,
    pub content: String,
}

pub fn open_database(path: &Path) -> Result<Connection> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("could not create database directory {}", parent.display()))?;
    }
    let connection = Connection::open(path)
        .with_context(|| format!("could not open database {}", path.display()))?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    connection.pragma_update(None, "synchronous", "NORMAL")?;
    migrate(&connection)?;
    Ok(connection)
}

pub fn migrate(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sources (
            id INTEGER PRIMARY KEY,
            scope TEXT NOT NULL,
            root_path TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            content_hash TEXT NOT NULL,
            content TEXT NOT NULL,
            byte_length INTEGER NOT NULL,
            active INTEGER NOT NULL DEFAULT 1 CHECK(active IN (0, 1)),
            created_at_ms INTEGER NOT NULL,
            updated_at_ms INTEGER NOT NULL,
            UNIQUE(scope, root_path, relative_path)
        );

        CREATE TABLE IF NOT EXISTS source_revisions (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
            content_hash TEXT NOT NULL,
            byte_length INTEGER NOT NULL,
            observed_at_ms INTEGER NOT NULL,
            UNIQUE(source_id, content_hash)
        );

        CREATE TABLE IF NOT EXISTS facts (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
            kind TEXT NOT NULL,
            subject TEXT NOT NULL,
            value TEXT NOT NULL,
            evidence TEXT NOT NULL,
            UNIQUE(source_id, kind, subject, value)
        );

        CREATE TABLE IF NOT EXISTS canonical_facts (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL,
            subject TEXT NOT NULL,
            value TEXT NOT NULL,
            UNIQUE(kind, subject, value)
        );

        CREATE TABLE IF NOT EXISTS fact_supports (
            fact_id INTEGER PRIMARY KEY REFERENCES facts(id) ON DELETE CASCADE,
            canonical_fact_id INTEGER NOT NULL REFERENCES canonical_facts(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS source_suppressions (
            scope TEXT NOT NULL,
            root_path TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            reason TEXT NOT NULL,
            suppressed_at_ms INTEGER NOT NULL,
            PRIMARY KEY(scope, root_path, relative_path)
        );

        CREATE TABLE IF NOT EXISTS operation_journal (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL CHECK(kind IN ('ingest', 'suppress', 'unsuppress')),
            scope TEXT NOT NULL,
            root_path TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            state TEXT NOT NULL CHECK(state IN ('started', 'completed', 'failed')),
            started_at_ms INTEGER NOT NULL,
            completed_at_ms INTEGER,
            error TEXT
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS source_fts USING fts5(
            source_id UNINDEXED,
            scope UNINDEXED,
            path,
            content,
            tokenize = 'unicode61 remove_diacritics 2'
        );

        CREATE INDEX IF NOT EXISTS sources_active_scope_idx ON sources(active, scope);
        CREATE INDEX IF NOT EXISTS facts_source_idx ON facts(source_id);
        CREATE INDEX IF NOT EXISTS fact_supports_canonical_idx ON fact_supports(canonical_fact_id);
        CREATE INDEX IF NOT EXISTS source_suppressions_scope_idx ON source_suppressions(scope);
        CREATE INDEX IF NOT EXISTS operation_journal_state_idx ON operation_journal(state, started_at_ms);
        "#,
    )?;
    let previous_schema: Option<String> = connection
        .query_row(
            "SELECT value FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .optional()?;
    if previous_schema.as_deref() == Some("3") {
        connection.execute_batch(
            r#"
            CREATE TABLE operation_journal_next (
                id INTEGER PRIMARY KEY,
                kind TEXT NOT NULL CHECK(kind IN ('ingest', 'suppress', 'unsuppress')),
                scope TEXT NOT NULL,
                root_path TEXT NOT NULL,
                relative_path TEXT NOT NULL,
                state TEXT NOT NULL CHECK(state IN ('started', 'completed', 'failed')),
                started_at_ms INTEGER NOT NULL,
                completed_at_ms INTEGER,
                error TEXT
            );
            INSERT INTO operation_journal_next(
                id, kind, scope, root_path, relative_path, state,
                started_at_ms, completed_at_ms, error
            )
            SELECT id, kind, scope, root_path, relative_path, state,
                   started_at_ms, completed_at_ms, error
            FROM operation_journal;
            DROP TABLE operation_journal;
            ALTER TABLE operation_journal_next RENAME TO operation_journal;
            CREATE INDEX IF NOT EXISTS operation_journal_state_idx
                ON operation_journal(state, started_at_ms);
            "#,
        )?;
    }
    connection.execute(
        "INSERT INTO metadata(key, value) VALUES ('schema_version', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [SCHEMA_VERSION],
    )?;
    Ok(())
}

pub fn ingest(connection: &mut Connection, root: &Path, scope: &str) -> Result<IngestSummary> {
    if scope.trim().is_empty() {
        bail!("scope must not be empty");
    }
    let canonical_root = canonical_root(root)?;
    let root_name = canonical_root.to_string_lossy().replace('\\', "/");
    let operation_id = start_operation(connection, "ingest", scope, &root_name, "")?;
    let result = ingest_transaction(connection, &canonical_root, scope, operation_id);
    finish_operation(connection, operation_id, &result)?;
    result.map(|mut summary| {
        summary.operation_id = operation_id;
        summary
    })
}

fn ingest_transaction(
    connection: &mut Connection,
    root: &Path,
    scope: &str,
    operation_id: i64,
) -> Result<IngestSummary> {
    if scope.trim().is_empty() {
        bail!("scope must not be empty");
    }
    let canonical_root = root
        .canonicalize()
        .with_context(|| format!("could not resolve input root {}", root.display()))?;
    if !canonical_root.is_dir() {
        bail!(
            "input root must be a directory: {}",
            canonical_root.display()
        );
    }

    let root_name = canonical_root.to_string_lossy().replace('\\', "/");
    let mut summary = IngestSummary::default();
    let mut seen = HashSet::new();
    let transaction = connection.transaction()?;

    for entry in WalkDir::new(&canonical_root)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                summary.skipped += 1;
                continue;
            }
        };
        if !entry.file_type().is_file() || entry.file_type().is_symlink() {
            continue;
        }
        if !is_supported_file(entry.path()) {
            summary.skipped += 1;
            continue;
        }

        let relative_path = entry
            .path()
            .strip_prefix(&canonical_root)
            .context("walk entry escaped the canonical input root")?
            .to_string_lossy()
            .replace('\\', "/");
        summary.discovered += 1;
        seen.insert(relative_path.clone());
        if is_suppressed(&transaction, scope, &root_name, &relative_path)? {
            summary.suppressed += 1;
            summary.revoked += deactivate_source(&transaction, scope, &root_name, &relative_path)?;
            continue;
        }
        let content = match fs::read_to_string(entry.path()) {
            Ok(content) => content,
            Err(_) => {
                summary.skipped += 1;
                continue;
            }
        };
        let change = upsert_source(&transaction, scope, &root_name, &relative_path, &content)?;
        match change {
            Change::Inserted => summary.inserted += 1,
            Change::Updated => summary.updated += 1,
            Change::Unchanged => summary.unchanged += 1,
        }
    }

    summary.revoked += revoke_missing_sources(&transaction, scope, &root_name, &seen)?;
    rebuild_canonical_facts(&transaction)?;
    summary.facts = transaction.query_row("SELECT COUNT(*) FROM facts", [], |row| row.get(0))?;
    complete_operation(&transaction, operation_id)?;
    transaction.commit()?;
    Ok(summary)
}

pub fn suppress_source(
    connection: &mut Connection,
    root: &Path,
    scope: &str,
    relative_path: &str,
    reason: &str,
) -> Result<Value> {
    if scope.trim().is_empty() || reason.trim().is_empty() {
        bail!("scope and reason must not be empty");
    }
    let root_path = canonical_root(root)?;
    let root_name = root_path.to_string_lossy().replace('\\', "/");
    let relative_path = normalized_relative_path(relative_path)?;
    let now = now_ms()?;
    let operation_id = start_operation(connection, "suppress", scope, &root_name, &relative_path)?;
    let result = (|| -> Result<Value> {
        let transaction = connection.transaction()?;
        transaction.execute(
            "INSERT INTO source_suppressions(scope, root_path, relative_path, reason, suppressed_at_ms)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(scope, root_path, relative_path)
             DO UPDATE SET reason = excluded.reason, suppressed_at_ms = excluded.suppressed_at_ms",
            params![scope, root_name, relative_path, reason, now],
        )?;
        let deactivated = deactivate_source(&transaction, scope, &root_name, &relative_path)?;
        rebuild_canonical_facts(&transaction)?;
        complete_operation(&transaction, operation_id)?;
        transaction.commit()?;
        Ok(json!({
            "operation_id": operation_id,
            "scope": scope,
            "root": display_root_path(&root_path),
            "path": relative_path,
            "reason": reason,
            "deactivated": deactivated == 1,
            "reingestion": "blocked until unsuppress followed by ingest",
        }))
    })();
    finish_operation(connection, operation_id, &result)?;
    result
}

pub fn unsuppress_source(
    connection: &mut Connection,
    root: &Path,
    scope: &str,
    relative_path: &str,
) -> Result<Value> {
    if scope.trim().is_empty() {
        bail!("scope must not be empty");
    }
    let root_path = canonical_root(root)?;
    let root_name = root_path.to_string_lossy().replace('\\', "/");
    let relative_path = normalized_relative_path(relative_path)?;
    let operation_id =
        start_operation(connection, "unsuppress", scope, &root_name, &relative_path)?;
    let result = (|| -> Result<Value> {
        let transaction = connection.transaction()?;
        let removed = transaction.execute(
            "DELETE FROM source_suppressions WHERE scope = ?1 AND root_path = ?2 AND relative_path = ?3",
            params![scope, root_name, relative_path],
        )?;
        complete_operation(&transaction, operation_id)?;
        transaction.commit()?;
        Ok(json!({
            "operation_id": operation_id,
            "scope": scope,
            "root": display_root_path(&root_path),
            "path": relative_path,
            "suppression_removed": removed == 1,
            "reingestion": "run ingest to make a present source searchable again",
        }))
    })();
    finish_operation(connection, operation_id, &result)?;
    result
}

pub fn search(
    connection: &Connection,
    query: &str,
    scope: &str,
    max_results: usize,
) -> Result<Vec<SearchResult>> {
    if query.trim().is_empty() {
        bail!("query must not be empty");
    }
    if max_results == 0 {
        return Ok(Vec::new());
    }
    let fts_query = safe_fts_query(query);
    let mut statement = connection.prepare(
        r#"
        SELECT s.id, s.relative_path, bm25(source_fts) AS score,
               snippet(source_fts, 3, '[', ']', '…', 24), s.content
        FROM source_fts
        JOIN sources s ON s.id = CAST(source_fts.source_id AS INTEGER)
        WHERE source_fts MATCH ?1 AND s.scope = ?2 AND s.active = 1
        ORDER BY score, s.relative_path
        LIMIT ?3
        "#,
    )?;
    let rows = statement.query_map(params![fts_query, scope, max_results as i64], |row| {
        Ok(SearchResult {
            source_id: row.get(0)?,
            path: row.get(1)?,
            score: row.get(2)?,
            snippet: row.get(3)?,
            content: row.get(4)?,
        })
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

pub fn inspect_source(connection: &Connection, source_id: i64) -> Result<Value> {
    let mut statement = connection.prepare(
        r#"
        SELECT id, scope, root_path, relative_path, content_hash, byte_length, active,
               created_at_ms, updated_at_ms, content
        FROM sources WHERE id = ?1
        "#,
    )?;
    let source = statement
        .query_row([source_id], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "scope": row.get::<_, String>(1)?,
                "root_path": row.get::<_, String>(2)?,
                "path": row.get::<_, String>(3)?,
                "content_hash": row.get::<_, String>(4)?,
                "byte_length": row.get::<_, i64>(5)?,
                "active": row.get::<_, i64>(6)? == 1,
                "created_at_ms": row.get::<_, i64>(7)?,
                "updated_at_ms": row.get::<_, i64>(8)?,
                "content": row.get::<_, String>(9)?,
            }))
        })
        .optional()?;
    let mut source = source.ok_or_else(|| anyhow::anyhow!("source {} was not found", source_id))?;

    let revisions = {
        let mut statement = connection.prepare(
            "SELECT content_hash, byte_length, observed_at_ms
             FROM source_revisions WHERE source_id = ?1 ORDER BY observed_at_ms, id",
        )?;
        statement
            .query_map([source_id], |row| {
                Ok(json!({
                    "content_hash": row.get::<_, String>(0)?,
                    "byte_length": row.get::<_, i64>(1)?,
                    "observed_at_ms": row.get::<_, i64>(2)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?
    };

    let facts = {
        let mut statement = connection.prepare(
            "SELECT f.id, f.kind, f.subject, f.value, f.evidence, c.id
             FROM facts f
             LEFT JOIN fact_supports fs ON fs.fact_id = f.id
             LEFT JOIN canonical_facts c ON c.id = fs.canonical_fact_id
             WHERE f.source_id = ?1 ORDER BY f.id",
        )?;
        statement
            .query_map([source_id], |row| {
                Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "kind": row.get::<_, String>(1)?,
                    "subject": row.get::<_, String>(2)?,
                    "value": row.get::<_, String>(3)?,
                    "evidence": row.get::<_, String>(4)?,
                    "canonical_fact_id": row.get::<_, Option<i64>>(5)?,
                }))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?
    };

    let (scope, root_path, relative_path): (String, String, String) = connection.query_row(
        "SELECT scope, root_path, relative_path FROM sources WHERE id = ?1",
        [source_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let suppression: Option<String> = connection
        .query_row(
            "SELECT reason FROM source_suppressions
             WHERE scope = ?1 AND root_path = ?2 AND relative_path = ?3",
            params![scope, root_path, relative_path],
            |row| row.get(0),
        )
        .optional()?;
    let fts_indexed: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM source_fts WHERE CAST(source_id AS INTEGER) = ?1)",
        [source_id],
        |row| row.get(0),
    )?;

    let object = source
        .as_object_mut()
        .expect("source inspection must be a JSON object");
    object.insert("revisions".to_owned(), json!(revisions));
    object.insert("facts".to_owned(), json!(facts));
    object.insert("suppressed".to_owned(), json!(suppression.is_some()));
    object.insert("suppression_reason".to_owned(), json!(suppression));
    object.insert("fts_indexed".to_owned(), json!(fts_indexed));
    Ok(source)
}

pub fn status(connection: &Connection) -> Result<Value> {
    let sources: i64 =
        connection.query_row("SELECT COUNT(*) FROM sources", [], |row| row.get(0))?;
    let active_sources: i64 =
        connection.query_row("SELECT COUNT(*) FROM sources WHERE active = 1", [], |row| {
            row.get(0)
        })?;
    let facts: i64 = connection.query_row("SELECT COUNT(*) FROM facts", [], |row| row.get(0))?;
    let canonical_facts: i64 =
        connection.query_row("SELECT COUNT(*) FROM canonical_facts", [], |row| row.get(0))?;
    let suppressed_sources: i64 =
        connection.query_row("SELECT COUNT(*) FROM source_suppressions", [], |row| {
            row.get(0)
        })?;
    let pending_operations: i64 = connection.query_row(
        "SELECT COUNT(*) FROM operation_journal WHERE state = 'started'",
        [],
        |row| row.get(0),
    )?;
    let schema_version: String = connection.query_row(
        "SELECT value FROM metadata WHERE key = 'schema_version'",
        [],
        |row| row.get(0),
    )?;
    Ok(json!({
        "schema_version": schema_version,
        "profile": "deterministic-lexical",
        "tokenizer": TOKENIZER_ID,
        "sources": sources,
        "active_sources": active_sources,
        "facts": facts,
        "canonical_facts": canonical_facts,
        "suppressed_sources": suppressed_sources,
        "pending_operations": pending_operations,
        "embedding_profile": null,
        "auxiliary_model": null,
    }))
}

pub fn verify(connection: &Connection) -> Result<Value> {
    let active_sources_missing_fts: i64 = connection.query_row(
        "SELECT COUNT(*) FROM sources s
         WHERE s.active = 1
           AND NOT EXISTS (
               SELECT 1 FROM source_fts f
               WHERE CAST(f.source_id AS INTEGER) = s.id
           )",
        [],
        |row| row.get(0),
    )?;
    let inactive_sources_in_fts: i64 = connection.query_row(
        "SELECT COUNT(*) FROM sources s
         WHERE s.active = 0
           AND EXISTS (
               SELECT 1 FROM source_fts f
               WHERE CAST(f.source_id AS INTEGER) = s.id
           )",
        [],
        |row| row.get(0),
    )?;
    let orphan_fts_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM source_fts f
         WHERE NOT EXISTS (
             SELECT 1 FROM sources s
             WHERE s.id = CAST(f.source_id AS INTEGER)
         )",
        [],
        |row| row.get(0),
    )?;
    let duplicate_fts_sources: i64 = connection.query_row(
        "SELECT COUNT(*) FROM (
             SELECT CAST(source_id AS INTEGER) AS source_id
             FROM source_fts
             GROUP BY CAST(source_id AS INTEGER)
             HAVING COUNT(*) > 1
         )",
        [],
        |row| row.get(0),
    )?;
    let facts_without_support: i64 = connection.query_row(
        "SELECT COUNT(*) FROM facts f
         LEFT JOIN fact_supports fs ON fs.fact_id = f.id
         WHERE fs.fact_id IS NULL",
        [],
        |row| row.get(0),
    )?;
    let canonical_without_support: i64 = connection.query_row(
        "SELECT COUNT(*) FROM canonical_facts c
         LEFT JOIN fact_supports fs ON fs.canonical_fact_id = c.id
         WHERE fs.canonical_fact_id IS NULL",
        [],
        |row| row.get(0),
    )?;
    let suppressed_active_sources: i64 = connection.query_row(
        "SELECT COUNT(*) FROM sources s
         JOIN source_suppressions ss
           ON ss.scope = s.scope
          AND ss.root_path = s.root_path
          AND ss.relative_path = s.relative_path
         WHERE s.active = 1",
        [],
        |row| row.get(0),
    )?;
    let pending_operations: i64 = connection.query_row(
        "SELECT COUNT(*) FROM operation_journal WHERE state = 'started'",
        [],
        |row| row.get(0),
    )?;
    let failed_operations: i64 = connection.query_row(
        "SELECT COUNT(*) FROM operation_journal WHERE state = 'failed'",
        [],
        |row| row.get(0),
    )?;
    let schema_version: String = connection.query_row(
        "SELECT value FROM metadata WHERE key = 'schema_version'",
        [],
        |row| row.get(0),
    )?;
    let checks = json!({
        "active_sources_missing_fts": active_sources_missing_fts,
        "inactive_sources_in_fts": inactive_sources_in_fts,
        "orphan_fts_rows": orphan_fts_rows,
        "duplicate_fts_sources": duplicate_fts_sources,
        "facts_without_support": facts_without_support,
        "canonical_without_support": canonical_without_support,
        "suppressed_active_sources": suppressed_active_sources,
        "pending_operations": pending_operations,
    });
    let ok = checks
        .as_object()
        .expect("integrity checks must be a JSON object")
        .values()
        .all(|value| value.as_i64() == Some(0));
    Ok(json!({
        "schema_version": schema_version,
        "profile": "deterministic-lexical",
        "ok": ok,
        "checks": checks,
        "failed_operations": failed_operations,
        "repair": "read-only; rerun ingest or use explicit suppression commands after reviewing failures",
    }))
}

pub fn list_operations(connection: &Connection, limit: usize) -> Result<Value> {
    let mut statement = connection.prepare(
        "SELECT id, kind, scope, root_path, relative_path, state, started_at_ms, completed_at_ms, error
         FROM operation_journal ORDER BY id DESC LIMIT ?1",
    )?;
    let operations = statement
        .query_map([limit as i64], |row| {
            let root: String = row.get(3)?;
            let relative_path: String = row.get(4)?;
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "kind": row.get::<_, String>(1)?,
                "scope": row.get::<_, String>(2)?,
                "root": display_root_path(Path::new(&root)),
                "path": if relative_path.is_empty() { Value::Null } else { json!(relative_path) },
                "state": row.get::<_, String>(5)?,
                "started_at_ms": row.get::<_, i64>(6)?,
                "completed_at_ms": row.get::<_, Option<i64>>(7)?,
                "error": row.get::<_, Option<String>>(8)?,
            }))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(json!({ "operations": operations }))
}

pub fn recover_operations(connection: &Connection, limit: usize) -> Result<Value> {
    if limit == 0 {
        return Ok(json!({
            "recovered": 0,
            "operations": [],
            "action": "marked_interrupted_operations_failed",
        }));
    }

    let pending = {
        let mut statement = connection.prepare(
            "SELECT id, kind, scope, root_path, relative_path
             FROM operation_journal WHERE state = 'started'
             ORDER BY id LIMIT ?1",
        )?;
        statement
            .query_map([limit as i64], |row| {
                let root: String = row.get(3)?;
                let relative_path: String = row.get(4)?;
                Ok((
                    row.get::<_, i64>(0)?,
                    json!({
                        "id": row.get::<_, i64>(0)?,
                        "kind": row.get::<_, String>(1)?,
                        "scope": row.get::<_, String>(2)?,
                        "root": display_root_path(Path::new(&root)),
                        "path": if relative_path.is_empty() { Value::Null } else { json!(relative_path) },
                    }),
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?
    };

    if pending.is_empty() {
        return Ok(json!({
            "recovered": 0,
            "operations": [],
            "action": "marked_interrupted_operations_failed",
        }));
    }

    let completed_at_ms = now_ms()?;
    let error =
        "operation was left started; rerun the original command after reviewing the database";
    let mut operations = Vec::with_capacity(pending.len());
    for (id, mut operation) in pending {
        connection.execute(
            "UPDATE operation_journal
             SET state = 'failed', completed_at_ms = ?1, error = ?2
             WHERE id = ?3 AND state = 'started'",
            params![completed_at_ms, error, id],
        )?;
        operation
            .as_object_mut()
            .expect("operation recovery result must be a JSON object")
            .insert("state".to_owned(), json!("failed"));
        operation
            .as_object_mut()
            .expect("operation recovery result must be a JSON object")
            .insert("error".to_owned(), json!(error));
        operations.push(operation);
    }

    Ok(json!({
        "recovered": operations.len(),
        "operations": operations,
        "action": "marked_interrupted_operations_failed",
        "retry": "rerun the original command after reviewing the database",
    }))
}

pub fn evaluate(
    connection: &Connection,
    fixture_path: &Path,
    scope: &str,
    max_results: usize,
) -> Result<Value> {
    let fixture = fs::read_to_string(fixture_path).with_context(|| {
        format!(
            "could not read evaluation fixture {}",
            fixture_path.display()
        )
    })?;
    let fixture: Value = serde_json::from_str(&fixture).with_context(|| {
        format!(
            "could not parse evaluation fixture {}",
            fixture_path.display()
        )
    })?;
    let queries = fixture
        .get("queries")
        .and_then(Value::as_array)
        .context("evaluation fixture must contain a queries array")?;
    if queries.is_empty() {
        bail!("evaluation fixture must contain at least one query");
    }

    let mut expected_total = 0usize;
    let mut found_total = 0usize;
    let mut reports = Vec::new();
    for entry in queries {
        let id = entry
            .get("id")
            .and_then(Value::as_str)
            .context("evaluation query is missing string id")?;
        let query = entry
            .get("query")
            .and_then(Value::as_str)
            .context("evaluation query is missing string query")?;
        let expected_paths = entry
            .get("expected_paths")
            .and_then(Value::as_array)
            .context("evaluation query is missing expected_paths array")?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .context("expected_paths must contain strings")
            })
            .collect::<Result<Vec<_>>>()?;
        if expected_paths.is_empty() {
            bail!("evaluation query {id} has no expected paths");
        }
        let expected = expected_paths.iter().cloned().collect::<HashSet<_>>();
        if expected.len() != expected_paths.len() {
            bail!("evaluation query {id} repeats an expected path");
        }
        for path in &expected_paths {
            let indexed: bool = connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM sources WHERE scope = ?1 AND relative_path = ?2 AND active = 1)",
                params![scope, path],
                |row| row.get(0),
            )?;
            if !indexed {
                bail!(
                    "evaluation preflight failed: expected path {path} is not active in scope {scope}"
                );
            }
        }

        let results = search(connection, query, scope, max_results)?;
        let returned_paths = results
            .iter()
            .map(|result| result.path.clone())
            .collect::<Vec<_>>();
        let found = returned_paths
            .iter()
            .filter(|path| expected.contains(*path))
            .count();
        expected_total += expected_paths.len();
        found_total += found;
        reports.push(json!({
            "id": id,
            "query": query,
            "expected_paths": expected_paths,
            "returned_paths": returned_paths,
            "relevant_found": found,
            "expected_count": expected.len(),
            "recall_at_k": found as f64 / expected.len() as f64,
        }));
    }

    Ok(json!({
        "fixture": fixture_path,
        "scope": scope,
        "profile": "deterministic-lexical",
        "max_results": max_results,
        "queries": reports,
        "expected_total": expected_total,
        "relevant_found": found_total,
        "recall_at_k": found_total as f64 / expected_total as f64,
    }))
}

pub fn context_package(
    query: &str,
    scope: &str,
    results: Vec<SearchResult>,
    budget: usize,
) -> Value {
    let mut used_tokens = 0usize;
    let mut evidence_items = Vec::new();
    let mut omissions = Vec::new();

    let mut pending = results.into_iter();
    while let Some(result) = pending.next() {
        let content = trim_to_budget(&result.content, budget.saturating_sub(used_tokens));
        let tokens = estimate_tokens(&content);
        if content.is_empty() && !result.content.trim().is_empty() {
            omissions.push(json!({
                "source_id": result.source_id,
                "path": result.path,
                "reason": "budget_exhausted"
            }));
            continue;
        }
        used_tokens += tokens;
        evidence_items.push(json!({
            "source_id": result.source_id,
            "path": result.path,
            "score": result.score,
            "snippet": result.snippet,
            "content": content,
            "estimated_tokens": tokens,
        }));
        if budget > 0 && used_tokens >= budget {
            omissions.extend(pending.map(|remaining| {
                json!({
                    "source_id": remaining.source_id,
                    "path": remaining.path,
                    "reason": "budget_exhausted"
                })
            }));
            break;
        }
    }
    let insufficient = evidence_items.is_empty() || (!omissions.is_empty() && budget > 0);
    json!({
        "generation": "single-writer-current",
        "query": query,
        "scope": scope,
        "profile": "deterministic-lexical",
        "tokenizer": TOKENIZER_ID,
        "budget": if budget == 0 { Value::Null } else { json!(budget) },
        "used_tokens": used_tokens,
        "evidence_items": evidence_items,
        "omissions": omissions,
        "unresolved_conflicts": [],
        "insufficient_context": insufficient,
        "degraded_reason": "No embedding, reranking, semantic librarian, or consumer tokenizer is configured in this deterministic profile.",
    })
}

fn upsert_source(
    transaction: &Transaction<'_>,
    scope: &str,
    root_path: &str,
    relative_path: &str,
    content: &str,
) -> Result<Change> {
    let hash = sha256(content);
    let now = now_ms()?;
    let existing: Option<(i64, String, i64)> = transaction
        .query_row(
            "SELECT id, content_hash, active FROM sources WHERE scope = ?1 AND root_path = ?2 AND relative_path = ?3",
            params![scope, root_path, relative_path],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()?;

    let (source_id, change) = match existing {
        None => {
            transaction.execute(
                "INSERT INTO sources(scope, root_path, relative_path, content_hash, content, byte_length, active, created_at_ms, updated_at_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?7)",
                params![scope, root_path, relative_path, hash, content, content.len() as i64, now],
            )?;
            (transaction.last_insert_rowid(), Change::Inserted)
        }
        Some((_source_id, previous_hash, active)) if previous_hash == hash && active == 1 => {
            return Ok(Change::Unchanged);
        }
        Some((source_id, _, _)) => {
            transaction.execute(
                "UPDATE sources SET content_hash = ?1, content = ?2, byte_length = ?3, active = 1, updated_at_ms = ?4 WHERE id = ?5",
                params![hash, content, content.len() as i64, now, source_id],
            )?;
            transaction.execute("DELETE FROM facts WHERE source_id = ?1", [source_id])?;
            transaction.execute(
                "DELETE FROM source_fts WHERE source_id = ?1",
                [source_id.to_string()],
            )?;
            (source_id, Change::Updated)
        }
    };

    transaction.execute(
        "INSERT OR IGNORE INTO source_revisions(source_id, content_hash, byte_length, observed_at_ms) VALUES (?1, ?2, ?3, ?4)",
        params![source_id, hash, content.len() as i64, now],
    )?;
    transaction.execute(
        "INSERT INTO source_fts(source_id, scope, path, content) VALUES (?1, ?2, ?3, ?4)",
        params![source_id.to_string(), scope, relative_path, content],
    )?;
    for fact in extract_composer_facts(relative_path, content) {
        transaction.execute(
            "INSERT OR IGNORE INTO facts(source_id, kind, subject, value, evidence) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![source_id, fact.kind, fact.subject, fact.value, fact.evidence],
        )?;
    }
    Ok(change)
}

fn revoke_missing_sources(
    transaction: &Transaction<'_>,
    scope: &str,
    root_path: &str,
    seen: &HashSet<String>,
) -> Result<usize> {
    let mut statement = transaction.prepare(
        "SELECT id, relative_path FROM sources WHERE scope = ?1 AND root_path = ?2 AND active = 1",
    )?;
    let active = statement
        .query_map(params![scope, root_path], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(statement);

    let now = now_ms()?;
    let mut revoked = 0;
    for (source_id, path) in active {
        if seen.contains(&path) {
            continue;
        }
        transaction.execute(
            "UPDATE sources SET active = 0, updated_at_ms = ?1 WHERE id = ?2",
            params![now, source_id],
        )?;
        transaction.execute("DELETE FROM facts WHERE source_id = ?1", [source_id])?;
        transaction.execute(
            "DELETE FROM source_fts WHERE source_id = ?1",
            [source_id.to_string()],
        )?;
        revoked += 1;
    }
    Ok(revoked)
}

fn is_suppressed(
    transaction: &Transaction<'_>,
    scope: &str,
    root_path: &str,
    relative_path: &str,
) -> Result<bool> {
    transaction
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM source_suppressions WHERE scope = ?1 AND root_path = ?2 AND relative_path = ?3)",
            params![scope, root_path, relative_path],
            |row| row.get(0),
        )
        .map_err(Into::into)
}

fn deactivate_source(
    transaction: &Transaction<'_>,
    scope: &str,
    root_path: &str,
    relative_path: &str,
) -> Result<usize> {
    let source_id: Option<i64> = transaction
        .query_row(
            "SELECT id FROM sources WHERE scope = ?1 AND root_path = ?2 AND relative_path = ?3 AND active = 1",
            params![scope, root_path, relative_path],
            |row| row.get(0),
        )
        .optional()?;
    let Some(source_id) = source_id else {
        return Ok(0);
    };
    transaction.execute(
        "UPDATE sources SET active = 0, updated_at_ms = ?1 WHERE id = ?2",
        params![now_ms()?, source_id],
    )?;
    transaction.execute("DELETE FROM facts WHERE source_id = ?1", [source_id])?;
    transaction.execute(
        "DELETE FROM source_fts WHERE source_id = ?1",
        [source_id.to_string()],
    )?;
    Ok(1)
}

fn start_operation(
    connection: &Connection,
    kind: &str,
    scope: &str,
    root_path: &str,
    relative_path: &str,
) -> Result<i64> {
    connection.execute(
        "INSERT INTO operation_journal(kind, scope, root_path, relative_path, state, started_at_ms)
         VALUES (?1, ?2, ?3, ?4, 'started', ?5)",
        params![kind, scope, root_path, relative_path, now_ms()?],
    )?;
    Ok(connection.last_insert_rowid())
}

fn finish_operation<T>(
    connection: &Connection,
    operation_id: i64,
    result: &Result<T>,
) -> Result<()> {
    match result {
        Ok(_) => {
            connection.execute(
                "UPDATE operation_journal SET state = 'completed', completed_at_ms = ?1 WHERE id = ?2",
                params![now_ms()?, operation_id],
            )?;
        }
        Err(error) => {
            connection.execute(
                "UPDATE operation_journal SET state = 'failed', completed_at_ms = ?1, error = ?2 WHERE id = ?3",
                params![now_ms()?, error.to_string(), operation_id],
            )?;
        }
    }
    Ok(())
}

fn complete_operation(transaction: &Transaction<'_>, operation_id: i64) -> Result<()> {
    transaction.execute(
        "UPDATE operation_journal
         SET state = 'completed', completed_at_ms = ?1
         WHERE id = ?2 AND state = 'started'",
        params![now_ms()?, operation_id],
    )?;
    Ok(())
}

fn rebuild_canonical_facts(transaction: &Transaction<'_>) -> Result<()> {
    transaction.execute("DELETE FROM fact_supports", [])?;
    transaction.execute("DELETE FROM canonical_facts", [])?;
    transaction.execute(
        "INSERT INTO canonical_facts(kind, subject, value)
         SELECT DISTINCT kind, subject, value FROM facts",
        [],
    )?;
    transaction.execute(
        "INSERT INTO fact_supports(fact_id, canonical_fact_id)
         SELECT f.id, c.id FROM facts f JOIN canonical_facts c
         ON c.kind = f.kind AND c.subject = f.subject AND c.value = f.value",
        [],
    )?;
    Ok(())
}

#[derive(Debug)]
struct Fact {
    kind: &'static str,
    subject: String,
    value: String,
    evidence: String,
}

fn extract_composer_facts(path: &str, content: &str) -> Vec<Fact> {
    if !path.ends_with("composer.json") {
        return Vec::new();
    }
    let Ok(value) = serde_json::from_str::<Value>(content) else {
        return Vec::new();
    };
    let mut facts = Vec::new();
    for field in ["require", "require-dev"] {
        let Some(packages) = value.get(field).and_then(Value::as_object) else {
            continue;
        };
        for (package, version) in packages {
            let Some(version) = version.as_str() else {
                continue;
            };
            facts.push(Fact {
                kind: "composer_requirement",
                subject: package.clone(),
                value: version.to_owned(),
                evidence: format!("{}.{}", field, package),
            });
        }
    }
    facts
}

fn is_supported_file(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|file_name| file_name.to_str()),
        Some("composer.json")
    ) || matches!(
        path.extension().and_then(|extension| extension.to_str()).map(str::to_ascii_lowercase),
        Some(extension) if matches!(extension.as_str(), "md" | "markdown" | "txt")
    )
}

fn canonical_root(root: &Path) -> Result<std::path::PathBuf> {
    let root = root
        .canonicalize()
        .with_context(|| format!("could not resolve input root {}", root.display()))?;
    if !root.is_dir() {
        bail!("input root must be a directory: {}", root.display());
    }
    Ok(root)
}

fn display_root_path(root: &Path) -> String {
    let path = root.to_string_lossy().replace('\\', "/");
    path.strip_prefix("//?/").unwrap_or(&path).to_owned()
}

fn normalized_relative_path(path: &str) -> Result<String> {
    let path = Path::new(path);
    if path.is_absolute() {
        bail!("source path must be relative to the input root");
    }
    let mut segments = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(segment) => segments.push(segment.to_string_lossy().to_string()),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                bail!("source path must not escape the input root")
            }
        }
    }
    if segments.is_empty() {
        bail!("source path must name a file relative to the input root");
    }
    Ok(segments.join("/"))
}

fn safe_fts_query(query: &str) -> String {
    query
        .split_whitespace()
        .map(|part| format!("\"{}\"", part.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" AND ")
}

fn trim_to_budget(content: &str, remaining_budget: usize) -> String {
    if remaining_budget == 0 {
        return content.to_owned();
    }
    let words = content.split_whitespace().collect::<Vec<_>>();
    if words.len() <= remaining_budget {
        words.join(" ")
    } else if remaining_budget == 1 {
        "…".to_owned()
    } else {
        format!("{} …", words[..remaining_budget - 1].join(" "))
    }
}

fn estimate_tokens(content: &str) -> usize {
    content.split_whitespace().count()
}

fn sha256(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}

fn now_ms() -> Result<i64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before Unix epoch")?
        .as_millis()
        .try_into()
        .context("millisecond timestamp does not fit i64")
}

enum Change {
    Inserted,
    Updated,
    Unchanged,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fts_query_quotes_each_term() {
        assert_eq!(safe_fts_query("uma versao"), "\"uma\" AND \"versao\"");
    }

    #[test]
    fn compressor_reports_word_budget() {
        assert_eq!(trim_to_budget("um dois tres", 2), "um …");
        assert_eq!(estimate_tokens(&trim_to_budget("um dois", 1)), 1);
        assert_eq!(estimate_tokens("um dois tres"), 3);
    }

    #[test]
    fn context_package_never_exceeds_budget() {
        let package = context_package(
            "consulta",
            "test",
            vec![
                SearchResult {
                    source_id: 1,
                    path: "first.md".to_owned(),
                    score: 1.0,
                    snippet: "um dois".to_owned(),
                    content: "um dois tres quatro".to_owned(),
                },
                SearchResult {
                    source_id: 2,
                    path: "second.md".to_owned(),
                    score: 2.0,
                    snippet: "cinco".to_owned(),
                    content: "cinco seis".to_owned(),
                },
            ],
            3,
        );
        assert_eq!(package["used_tokens"], 3);
        assert_eq!(package["evidence_items"].as_array().unwrap().len(), 1);
        assert_eq!(package["evidence_items"][0]["content"], "um dois …");
        assert_eq!(package["insufficient_context"], true);
    }

    #[test]
    fn composer_requirements_become_facts() {
        let facts = extract_composer_facts(
            "composer.json",
            r#"{"require":{"vendor/package":"^1.2"},"require-dev":{"test/tool":"^2"}}"#,
        );
        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0].subject, "vendor/package");
    }

    #[test]
    fn ingestion_is_incremental_and_revokes_missing_files() -> Result<()> {
        let test_root = std::env::temp_dir().join(format!("mnemvia-test-{}", now_ms()?));
        fs::create_dir_all(&test_root)?;
        fs::write(test_root.join("note.md"), "shared local knowledge")?;
        let database = test_root.join("index.sqlite3");
        let mut connection = open_database(&database)?;

        let first = ingest(&mut connection, &test_root, "test")?;
        assert_eq!(first.inserted, 1);
        assert_eq!(search(&connection, "shared knowledge", "test", 5)?.len(), 1);

        let second = ingest(&mut connection, &test_root, "test")?;
        assert_eq!(second.unchanged, 1);
        assert_eq!(second.updated, 0);

        fs::remove_file(test_root.join("note.md"))?;
        let third = ingest(&mut connection, &test_root, "test")?;
        assert_eq!(third.revoked, 1);
        assert!(search(&connection, "shared knowledge", "test", 5)?.is_empty());

        drop(connection);
        fs::remove_dir_all(&test_root)?;
        Ok(())
    }

    #[test]
    fn inspect_exposes_revisions_facts_suppression_and_fts_state() -> Result<()> {
        let test_root = std::env::temp_dir().join(format!("mnemvia-inspect-{}", now_ms()?));
        fs::create_dir_all(&test_root)?;
        fs::write(
            test_root.join("composer.json"),
            r#"{"require":{"vendor/package":"^1.2"}}"#,
        )?;
        let database = test_root.join("index.sqlite3");
        let mut connection = open_database(&database)?;
        ingest(&mut connection, &test_root, "test")?;
        let source_id = search(&connection, "vendor package", "test", 5)?
            .first()
            .context("expected composer source in search results")?
            .source_id;

        let inspected = inspect_source(&connection, source_id)?;
        assert_eq!(inspected["active"], true);
        assert_eq!(inspected["fts_indexed"], true);
        assert_eq!(inspected["suppressed"], false);
        assert_eq!(inspected["revisions"].as_array().unwrap().len(), 1);
        assert_eq!(inspected["facts"].as_array().unwrap().len(), 1);
        assert!(inspected["facts"][0]["canonical_fact_id"].is_number());

        suppress_source(
            &mut connection,
            &test_root,
            "test",
            "composer.json",
            "fixture-policy",
        )?;
        let suppressed = inspect_source(&connection, source_id)?;
        assert_eq!(suppressed["active"], false);
        assert_eq!(suppressed["suppressed"], true);
        assert_eq!(suppressed["suppression_reason"], "fixture-policy");
        assert_eq!(suppressed["fts_indexed"], false);

        drop(connection);
        fs::remove_dir_all(&test_root)?;
        Ok(())
    }

    #[test]
    fn suppression_blocks_reingestion_until_explicitly_removed() -> Result<()> {
        let test_root = std::env::temp_dir().join(format!("mnemvia-suppress-{}", now_ms()?));
        fs::create_dir_all(&test_root)?;
        fs::write(test_root.join("note.md"), "private shared knowledge")?;
        let database = test_root.join("index.sqlite3");
        let mut connection = open_database(&database)?;
        ingest(&mut connection, &test_root, "test")?;
        assert_eq!(
            search(&connection, "private knowledge", "test", 5)?.len(),
            1
        );

        let result = suppress_source(
            &mut connection,
            &test_root,
            "test",
            "note.md",
            "fixture-policy",
        )?;
        assert_eq!(result["deactivated"], true);
        assert!(search(&connection, "private knowledge", "test", 5)?.is_empty());

        let suppressed = ingest(&mut connection, &test_root, "test")?;
        assert_eq!(suppressed.suppressed, 1);
        assert_eq!(
            suppressed.inserted + suppressed.updated + suppressed.unchanged,
            0
        );

        unsuppress_source(&mut connection, &test_root, "test", "note.md")?;
        let restored = ingest(&mut connection, &test_root, "test")?;
        assert_eq!(restored.updated, 1);
        assert_eq!(
            search(&connection, "private knowledge", "test", 5)?.len(),
            1
        );
        let operations = list_operations(&connection, 10)?;
        let operations = operations["operations"]
            .as_array()
            .context("operations must be an array")?;
        assert!(operations.len() >= 2);
        assert!(
            operations
                .iter()
                .all(|operation| operation["state"] == "completed")
        );
        assert!(
            operations
                .iter()
                .any(|operation| operation["kind"] == "ingest")
        );
        assert!(
            operations
                .iter()
                .any(|operation| operation["kind"] == "suppress")
        );
        assert!(
            operations
                .iter()
                .any(|operation| operation["kind"] == "unsuppress")
        );

        drop(connection);
        fs::remove_dir_all(&test_root)?;
        Ok(())
    }

    #[test]
    fn suppression_rejects_path_escape() {
        assert!(normalized_relative_path("../private.md").is_err());
        assert!(normalized_relative_path("").is_err());
    }

    #[test]
    fn schema_v3_operation_journal_migrates_to_v4() -> Result<()> {
        let connection = Connection::open_in_memory()?;
        connection.execute_batch(
            r#"
            CREATE TABLE metadata (key TEXT PRIMARY KEY NOT NULL, value TEXT NOT NULL);
            INSERT INTO metadata(key, value) VALUES ('schema_version', '3');
            CREATE TABLE operation_journal (
                id INTEGER PRIMARY KEY,
                kind TEXT NOT NULL CHECK(kind IN ('suppress', 'unsuppress')),
                scope TEXT NOT NULL,
                root_path TEXT NOT NULL,
                relative_path TEXT NOT NULL,
                state TEXT NOT NULL CHECK(state IN ('started', 'completed', 'failed')),
                started_at_ms INTEGER NOT NULL,
                completed_at_ms INTEGER,
                error TEXT
            );
            INSERT INTO operation_journal(
                kind, scope, root_path, relative_path, state, started_at_ms
            ) VALUES ('suppress', 'test', '/tmp/root', 'note.md', 'completed', 1);
            "#,
        )?;

        migrate(&connection)?;

        let schema_version: String = connection.query_row(
            "SELECT value FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(schema_version, "4");
        assert_eq!(
            connection.query_row::<i64, _, _>(
                "SELECT COUNT(*) FROM operation_journal WHERE kind = 'suppress'",
                [],
                |row| row.get(0),
            )?,
            1
        );
        connection.execute(
            "INSERT INTO operation_journal(
                 kind, scope, root_path, relative_path, state, started_at_ms
             ) VALUES ('ingest', 'test', '/tmp/root', '', 'started', ?1)",
            [now_ms()?],
        )?;
        Ok(())
    }

    #[test]
    fn verify_reports_clean_state_and_detects_orphan_fts_rows() -> Result<()> {
        let connection = Connection::open_in_memory()?;
        migrate(&connection)?;

        let clean = verify(&connection)?;
        assert_eq!(clean["ok"], true);
        assert_eq!(clean["checks"]["orphan_fts_rows"], 0);

        connection.execute(
            "INSERT INTO source_fts(source_id, scope, path, content)
             VALUES ('999', 'test', 'orphan.md', 'orphan content')",
            [],
        )?;
        let inconsistent = verify(&connection)?;
        assert_eq!(inconsistent["ok"], false);
        assert_eq!(inconsistent["checks"]["orphan_fts_rows"], 1);
        Ok(())
    }

    #[test]
    fn recover_marks_started_operations_failed_without_mutating_sources() -> Result<()> {
        let connection = Connection::open_in_memory()?;
        migrate(&connection)?;
        let operation_id = start_operation(&connection, "ingest", "test", "/tmp/root", "")?;

        let recovered = recover_operations(&connection, 10)?;
        assert_eq!(recovered["recovered"], 1);
        assert_eq!(recovered["operations"][0]["id"], operation_id);
        assert_eq!(recovered["operations"][0]["state"], "failed");
        assert_eq!(
            connection.query_row::<i64, _, _>(
                "SELECT COUNT(*) FROM operation_journal WHERE state = 'started'",
                [],
                |row| row.get(0),
            )?,
            0
        );
        assert_eq!(
            connection
                .query_row::<i64, _, _>("SELECT COUNT(*) FROM sources", [], |row| row.get(0),)?,
            0
        );
        let repeated = recover_operations(&connection, 10)?;
        assert_eq!(repeated["recovered"], 0);
        assert!(repeated["operations"].as_array().is_some_and(Vec::is_empty));
        Ok(())
    }

    #[test]
    fn operation_completion_rolls_back_with_mutation() -> Result<()> {
        let mut connection = Connection::open_in_memory()?;
        migrate(&connection)?;
        let operation_id = start_operation(&connection, "ingest", "test", "/tmp/root", "")?;
        {
            let transaction = connection.transaction()?;
            transaction.execute(
                "INSERT INTO sources(
                    scope, root_path, relative_path, content_hash, content,
                    byte_length, active, created_at_ms, updated_at_ms
                 ) VALUES ('test', '/tmp/root', 'note.md', 'hash', 'content', 7, 1, 1, 1)",
                [],
            )?;
            complete_operation(&transaction, operation_id)?;
        }

        assert_eq!(
            connection
                .query_row::<i64, _, _>("SELECT COUNT(*) FROM sources", [], |row| row.get(0))?,
            0
        );
        assert_eq!(
            connection.query_row::<String, _, _>(
                "SELECT state FROM operation_journal WHERE id = ?1",
                [operation_id],
                |row| row.get(0),
            )?,
            "started"
        );
        Ok(())
    }

    #[test]
    fn evaluation_rejects_unindexed_expected_paths() -> Result<()> {
        let connection = Connection::open_in_memory()?;
        migrate(&connection)?;
        let fixture_path = std::env::temp_dir().join(format!("mnemvia-eval-{}.json", now_ms()?));
        fs::write(
            &fixture_path,
            r#"{"queries":[{"id":"missing","query":"anything","expected_paths":["missing.md"]}]}"#,
        )?;

        let error =
            evaluate(&connection, &fixture_path, "test", 3).expect_err("preflight must fail");
        assert!(error.to_string().contains("preflight failed"));
        fs::remove_file(fixture_path)?;
        Ok(())
    }
}
