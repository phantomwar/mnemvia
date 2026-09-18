# Mnemvia

**Reusable knowledge. Traceable context. Local execution.**

[Português](README.pt-BR.md) · [GitHub](https://github.com/phantomwar/mnemvia) · [Dependencies](DEPENDENCIES.md) · [Roadmap](ROADMAP.md) · [Contributing](CONTRIBUTING.md)

Mnemvia is an open-source local RAG engine designed to reuse knowledge across documents and compile traceable context within a token budget.

It is being designed for Markdown knowledge bases and project documentation: ingest changes incrementally, preserve shared concepts and local differences, retrieve relevant evidence, and deliver context to a separate AI consumer.

**Status: deterministic M0 implementation.** The repository contains a runnable local CLI plus specifications, research and policies. It has no published release, vector retrieval, semantic librarian, consumer tokenizer integration or measured performance claim.

## What it is designed to do

- Ingest local Markdown/text and one structured manifest/lockfile format.
- Reuse unchanged content and shared facts without merging different entities or versions.
- Preserve source revisions, evidence, variants, uncertainty and explicit temporal validity.
- Combine lexical, vector and relational retrieval within configured resource limits.
- Compile inspectable context with actual tokenizer accounting and explicit omissions.
- Update and delete derived knowledge safely, including recovery after interruptions.
- Return proposed Markdown changes for human review.

A typical case is two projects using the same library version with different settings. The shared definition can be reused; each project's usage, settings and evidence remain separate.

## Intended workflow

~~~mermaid
flowchart LR
    S[Local documents] --> I[Incremental ingestion]
    I --> K[Canonical knowledge and evidence]
    K --> R[Retrieval]
    R --> C[Context compiler]
    C --> P[Budgeted context package]
    P --> A[External AI consumer]
    A --> D[Reviewable Markdown proposal]
    D --> S
~~~

Canonical knowledge is the planned intermediate representation, called KIR in the PRD. It links reusable assertions to their sources; it does not make model extraction infallible. Sources are preserved, and conflicting or inferred claims remain distinguishable.

The semantic librarian is a logical processing role, not a requirement for two permanently loaded language models. The implemented deterministic lexical profile returns context only and does not run an answer generator.

## Run the deterministic M0 profile

Requires Rust 1.94 or later. The first Cargo build downloads the resolved build dependencies; ingestion and search work locally afterward and do not download models or send source content to a service.

~~~sh
cargo run -- init --database .mnemvia/mnemvia.sqlite3
cargo run -- ingest --database .mnemvia/mnemvia.sqlite3 --root fixtures/m0-corpus --scope demo
cargo run -- search --database .mnemvia/mnemvia.sqlite3 --scope demo --query "shared authentication" --budget 80
cargo run -- eval --database .mnemvia/mnemvia.sqlite3 --scope demo --fixture fixtures/m0-corpus/evaluation.json
cargo run -- suppress --database .mnemvia/mnemvia.sqlite3 --root fixtures/m0-corpus --scope demo --path project-atlas/README.md --reason "private source"
cargo run -- unsuppress --database .mnemvia/mnemvia.sqlite3 --root fixtures/m0-corpus --scope demo --path project-atlas/README.md
cargo run -- operations --database .mnemvia/mnemvia.sqlite3
cargo run -- recover --database .mnemvia/mnemvia.sqlite3
cargo run -- status --database .mnemvia/mnemvia.sqlite3
cargo run -- verify --database .mnemvia/mnemvia.sqlite3
~~~

The `search` output is a JSON ContextPackage with sources, a heuristic word budget and explicit degraded capabilities. `heuristic-v1` is not an exact tokenizer count. Use `inspect --source-id <id>` to retrieve preserved evidence, revision hashes, extracted facts, suppression state and FTS parity for a result. `eval` validates fixture preconditions and reports retrieval recall; it does not claim semantic answer quality or benchmark performance.

`suppress` removes a source from retrieval and records a persistent block on future ingestion under that root and scope. It does not delete the original file. `unsuppress` only removes the block; run `ingest` afterward to reactivate a present source.

The schema also records ingestion and suppression operations as `started`, `completed` or `failed`; use `operations` to inspect them. An ingestion operation with `path: null` represents the whole root. A pending operation makes an incomplete transition visible; automatic replay and full crash recovery for batch deletion remain future work.

`verify` is a read-only integrity check for source/FTS parity, fact support, suppression policy and pending operations. It reports whether the current database is internally consistent; it does not repair data automatically.

`recover` marks operations left in `started` state as failed with a retry instruction. It does not guess whether a crashed transaction committed; review the database and rerun the original command explicitly.

## Dependencies and related projects

The deterministic profile adopts Rust, bundled SQLite/FTS5 and the direct crates listed in [DEPENDENCIES.md](DEPENDENCIES.md). Models, tokenization libraries and the inference runtime remain to be selected through evaluation.

| Project | Planned relationship |
|---|---|
| QMD | Local retrieval baseline and candidate for composition |
| LightRAG | Shared-knowledge maintenance baseline/reference |
| Graphiti, Microsoft GraphRAG and Cognee | Focused research and possible evaluation |
| RAPTOR | Research reference for hierarchical retrieval |
| LLMLingua | Candidate for an optional compression experiment |

These projects are not a bundle of required dependencies. No integration or downstream adopter is confirmed. See the [dependency register](DEPENDENCIES.md) for upstream links, roles, execution profiles and adoption criteria.

An Obsidian vault is a possible source folder; the Obsidian application is not required by the planned core. There is no mandatory cloud service or paid API in the target local workflow.

## Scope and validation

The initial delivery targets a CLI and loopback API. Infrastructure diagnostics can consume its context, but host monitoring and command execution are outside the core scope. PDF/OCR, audio and web crawling are outside the first alpha.

The central experiment compares a strong simple baseline, QMD, canonical knowledge and optional semantic compression. Measure quality, tokens, memory, latency, initial indexing and incremental maintenance together. A smaller prompt alone does not prove a better system.

Before building a complete retrieval engine, evaluate whether an existing engine plus Mnemvia's provenance and compilation layer meets the requirements.

## Documentation

- [Documentation guide](docs/README.md): current specifications and historical material.
- [PRD](docs/02-prd.md): requirements, data contracts and acceptance criteria.
- [Execution plan](docs/03-plano-de-execucao.md): phases, dependencies and experiments.
- [Comparative research](docs/07-pesquisa-comparativa-e-prevencao.md): evidence, correlations and proposed failure prevention.
- [Dependency register](DEPENDENCIES.md): candidates versus adopted dependencies.
- [Open-source plan](docs/05-plano-open-source.md): distribution and community strategy.

Detailed planning is currently in Portuguese. Public entry documents are in English, with a Portuguese README. Both languages are welcome in discussions.

## Contributing and security

Useful contributions now include synthetic corpora, questions with expected evidence, canonicalization edge cases and evaluation designs. Follow [CONTRIBUTING.md](CONTRIBUTING.md), [governance](GOVERNANCE.md) and the [code of conduct](CODE_OF_CONDUCT.md).

Do not submit private documents, secrets or unlicensed datasets. The [security policy](SECURITY.md) states the current reporting limitations and requirements before a public alpha.

## License

Original project materials are licensed under [Apache-2.0](LICENSE), unless explicitly stated otherwise. Third-party components, input data and model weights retain their own terms. See [licensing policy](docs/LICENSING.md).

Mnemvia is the selected project name. The public repository is [github.com/phantomwar/mnemvia](https://github.com/phantomwar/mnemvia). No trademark, domain or package namespace was registered by this planning work.

