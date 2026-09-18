# Roadmap

Mnemvia is an open-source local knowledge engine and optimized RAG context compiler. The deterministic M0 CLI is implemented and unreleased; later milestones remain gated by evidence.

## M0 — Establish evidence (implemented baseline, release pending)
- Redistributable document corpus, shared concepts, variants, changes and unanswerable queries.
- Deterministic local ingestion, SQLite/FTS5 retrieval, source revisions, suppression and operation journal.
- Read-only integrity verification for source/FTS parity, fact support and pending operations.
- Reproducible evaluation fixture and a simple lexical baseline; competitive embedding baselines remain pending.
- Record resource costs, retrieval quality and actual tokenizer counts.
- Compare against QMD and decide whether to compose existing retrieval or build a new engine. Reject invalid benchmark setups explicitly.

## M1 — Canonical and incremental knowledge (next implementation increment)
- Local ingestion, source revisions and provenance.
- Reusable entities with per-document/project variants and overrides.
- Dependency-aware invalidation, deletion, scope isolation and recovery.
- Durable operation state, immediate revocation, explicit temporal validity and embedding compatibility; test crash recovery and out-of-order ingestion.

## M2 — Retrieval and context compilation
- Hybrid retrieval and bounded relation expansion.
- Budgeted, inspectable context packages preserving evidence and constraints.
- Optional local semantic librarian with validated output and deterministic fallback.
- Compare quality, tokens and total lifecycle cost against the strongest simple baseline.
- Evaluate Portuguese/English separately, bound model retries, and report degraded or incomplete context honestly.

## M3 — Usable local alpha
- CLI and loopback API; context-only operation without a generator.
- Human-reviewed Markdown proposals and re-ingestion.
- Offline demo, tested installation, backup/migrations and release security checks.
- Verify that restoring an old backup cannot reactivate revoked source access.

Each milestone is gated by evidence, not a promised date. Additional parsers, rerankers, hierarchical summaries and index optimizations follow measured demand. Infrastructure agents and operational execution are independent consumers outside the core roadmap.

See the [PRD](docs/02-prd.md), [execution plan](docs/03-plano-de-execucao.md) and [corrected scope](docs/06-foco-original.md) for acceptance criteria.

The [comparative research](docs/07-pesquisa-comparativa-e-prevencao.md) records seven related projects/techniques and the evidence behind these prevention requirements. No comparative benchmarks have been executed yet.



## Dependency and documentation gates

The [dependency register](DEPENDENCIES.md) contains adopted M0 build dependencies alongside candidates and research relationships. M0 records selected versions, roles, licenses and platform requirements in the manifest, lockfile and register. Later adoption updates manifests/lockfiles, the register and both READMEs together.

Mnemvia is an open-source local RAG engine designed to reuse knowledge across documents and compile traceable context within a token budget.

