use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use mnemvia_cli::{
    context_package, evaluate, ingest, inspect_source, list_operations, open_database,
    recover_operations, search, status, suppress_source, unsuppress_source, verify,
};

#[derive(Debug, Parser)]
#[command(
    name = "mnemvia",
    version,
    about = "Local deterministic ingestion and retrieval"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create or migrate a local Mnemvia database.
    Init {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
    },
    /// Ingest supported files from a local directory without following symlinks.
    Ingest {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long, default_value = "default")]
        scope: String,
    },
    /// Search active source evidence and emit a context-only package.
    Search {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "default")]
        scope: String,
        #[arg(long, default_value_t = 10)]
        max_results: usize,
        /// Word-based temporary budget. Zero means no local truncation.
        #[arg(long, default_value_t = 0)]
        budget: usize,
    },
    /// Run a retrieval fixture after checking that expected files are indexed.
    Eval {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long, default_value = "default")]
        scope: String,
        #[arg(long, default_value_t = 10)]
        max_results: usize,
    },
    /// Persistently block a source from retrieval and future ingestion.
    Suppress {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        path: String,
        #[arg(long, default_value = "default")]
        scope: String,
        #[arg(long, default_value = "user-requested")]
        reason: String,
    },
    /// Remove a persistent suppression; ingest must be run separately to reactivate content.
    Unsuppress {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        path: String,
        #[arg(long, default_value = "default")]
        scope: String,
    },
    /// List recent ingestion and persistent suppression operations.
    Operations {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Mark interrupted operations as failed so they can be reviewed and retried.
    Recover {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
    /// Inspect an indexed source and its preserved content.
    Inspect {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
        #[arg(long)]
        source_id: i64,
    },
    /// Show database state and enabled capabilities.
    Status {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
    },
    /// Verify source, FTS, fact-support and operation invariants without mutating the database.
    Verify {
        #[arg(long, default_value = ".mnemvia/mnemvia.sqlite3")]
        database: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { database } => {
            let connection = open_database(&database)?;
            println!("{}", serde_json::to_string_pretty(&status(&connection)?)?);
        }
        Command::Ingest {
            database,
            root,
            scope,
        } => {
            let mut connection = open_database(&database)?;
            let summary = ingest(&mut connection, &root, &scope)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "scope": scope,
                    "root": root,
                    "operation_id": summary.operation_id,
                    "discovered": summary.discovered,
                    "inserted": summary.inserted,
                    "updated": summary.updated,
                    "unchanged": summary.unchanged,
                    "revoked": summary.revoked,
                    "suppressed": summary.suppressed,
                    "skipped": summary.skipped,
                    "facts": summary.facts,
                }))?
            );
        }
        Command::Search {
            database,
            query,
            scope,
            max_results,
            budget,
        } => {
            let connection = open_database(&database)?;
            let results = search(&connection, &query, &scope, max_results)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&context_package(&query, &scope, results, budget))?
            );
        }
        Command::Eval {
            database,
            fixture,
            scope,
            max_results,
        } => {
            let connection = open_database(&database)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&evaluate(
                    &connection,
                    &fixture,
                    &scope,
                    max_results
                )?)?
            );
        }
        Command::Suppress {
            database,
            root,
            path,
            scope,
            reason,
        } => {
            let mut connection = open_database(&database)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&suppress_source(
                    &mut connection,
                    &root,
                    &scope,
                    &path,
                    &reason,
                )?)?
            );
        }
        Command::Unsuppress {
            database,
            root,
            path,
            scope,
        } => {
            let mut connection = open_database(&database)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&unsuppress_source(
                    &mut connection,
                    &root,
                    &scope,
                    &path,
                )?)?
            );
        }
        Command::Operations { database, limit } => {
            let connection = open_database(&database)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&list_operations(&connection, limit)?)?
            );
        }
        Command::Recover { database, limit } => {
            let connection = open_database(&database)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&recover_operations(&connection, limit)?)?
            );
        }
        Command::Inspect {
            database,
            source_id,
        } => {
            let connection = open_database(&database)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&inspect_source(&connection, source_id)?)?
            );
        }
        Command::Status { database } => {
            let connection = open_database(&database)?;
            println!("{}", serde_json::to_string_pretty(&status(&connection)?)?);
        }
        Command::Verify { database } => {
            let connection = open_database(&database)?;
            println!("{}", serde_json::to_string_pretty(&verify(&connection)?)?);
        }
    }
    Ok(())
}
