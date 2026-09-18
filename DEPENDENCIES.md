# Dependencies and related projects

Mnemvia is an open-source local RAG engine designed to reuse knowledge across documents and compile traceable context within a token budget.

**Status: deterministic M0 implementation.** The Rust CLI has a resolved manifest and lockfile. Its lexical/local profile has no inference runtime, embedding model, external service, integration or confirmed downstream application. This register distinguishes the installed build from future candidates; it is not a software bill of materials.

## Dependency categories

- **Adopted:** present in the implementation, pinned/resolved in the build and tested.
- **Candidate:** proposed for evaluation; installation is not required today.
- **Evaluation-only:** a baseline or experiment, not a production prerequisite.
- **Research reference:** informs the design; no code dependency or affiliation implied.
- **Consumer:** a separate application that may call Mnemvia. No confirmed downstream projects today.

## Adopted build dependencies

The exact resolved dependency graph is [Cargo.lock](Cargo.lock); the direct packages are declared in [Cargo.toml](Cargo.toml). Versions below reflect the initial lockfile and must be updated with it.

| Component | Locked version | Role | Profile |
|---|---:|---|---|
| Rust | 1.94.0 toolchain used for validation | CLI and deterministic core | `deterministic-lexical` |
| [rusqlite](https://github.com/rusqlite/rusqlite) | 0.37.0 | Embedded SQLite, transactions and FTS5 | `deterministic-lexical` |
| SQLite | bundled by `libsqlite3-sys` 0.35.0 | Source store, revisions, facts and FTS5 index | `deterministic-lexical` |
| [clap](https://github.com/clap-rs/clap) | 4.6.7 | Typed CLI commands | `deterministic-lexical` |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5.0 | Recursive source enumeration without followed links | `deterministic-lexical` |
| [sha2](https://github.com/RustCrypto/hashes) | 0.10.9 | SHA-256 revision identity | `deterministic-lexical` |
| [serde_json](https://github.com/serde-rs/json) | 1.0.151 | Composer parsing and JSON ContextPackage output | `deterministic-lexical` |
| [anyhow](https://github.com/dtolnay/anyhow) | 1.0.104 | Contextual CLI errors | `deterministic-lexical` |

`rusqlite` uses its bundled SQLite feature for this initial build, avoiding a separately installed system SQLite dependency. The FTS5 capability is verified by the executable smoke test. Licenses/notices, SBOM generation and release artifact provenance remain release gates; this table does not replace them.

## Future implementation components

| Component | Status | Intended role | Decision required |
|---|---|---|---|
| Rust | Adopted for M0 deterministic CLI | Native core/CLI | Re-evaluate build-versus-compose after baseline results |
| SQLite | Adopted for M0 deterministic store | Sources, revisions, facts and lexical FTS | Validate lifecycle/recovery semantics before broader scope |
| Local inference runtime | Candidate, implementation not selected | Execute embedding and auxiliary models through separate adapters | Resource measurements, offline setup and model compatibility |
| Embedding model | Candidate, model not selected | Required for the semantic/vector profile | PT-BR/English quality, memory, license, immutable revision |
| Auxiliary language model | Optional candidate, not selected | Semantic librarian and ambiguous extraction | Schema/evidence validation, latency and bounded retries |
| Consumer tokenizer | Required capability for token-budgeted output; library not selected | Count the actual serialized context | Exact tokenizer/revision and compatibility with the consumer |
| Vector index library/extension | Candidate, not selected | Search within one compatible embedding space | Exact-versus-approximate evaluation, recall and migration |
| Parser libraries | Candidates, not selected | Markdown/text and one structured manifest/lockfile format | Correctness, provenance spans, license and platform support |

Names without an implementation selected are open decisions, not missing installation instructions. Do not install all research projects to use this repository.

## Related projects and precise relationships

| Project | Relationship to Mnemvia | Intended use | Production dependency today? |
|---|---|---|---|
| [QMD](https://github.com/tobi/qmd) | Evaluation baseline and composition candidate | Compare local retrieval; determine whether to reuse it behind an adapter | No |
| [LightRAG](https://github.com/HKUDS/LightRAG) | Evaluation/reference | Compare shared-knowledge maintenance and source deletion | No |
| [Graphiti](https://github.com/getzep/graphiti) | Research reference; possible focused evaluation | Temporal validity, provenance and conflicting facts | No |
| [Microsoft GraphRAG](https://github.com/microsoft/graphrag) | Research reference; possible focused evaluation | Global synthesis and indexing trade-offs | No |
| [Cognee](https://github.com/topoteretes/cognee) | Research reference; possible focused evaluation | Memory pipelines and access boundaries | No |
| [RAPTOR](https://arxiv.org/abs/2401.18059) | Research reference | Hierarchical retrieval experiment | No |
| [LLMLingua](https://github.com/microsoft/LLMLingua) | Optional compression experiment candidate | Compare compressed and extractive context at equal retrieval | No |

These roles are Mnemvia planning decisions. The [comparative research](docs/07-pesquisa-comparativa-e-prevencao.md) documents capabilities and limitations with primary sources. A listed project is not a partner, a fork parent or a required package merely because it inspired the design.

If the QMD composition experiment succeeds, record the selected integration boundary and actual transitive runtime dependencies before calling it adopted. If a dedicated core is selected, QMD can remain evaluation-only. Do not present both alternatives as an already implemented architecture.

## Execution profiles

| Planned profile | Required capabilities | Intentionally optional |
|---|---|---|
| Deterministic local retrieval | Parsers, source store and lexical search | Embeddings, auxiliary LLM and answer generator |
| Hybrid context compilation | Deterministic core, embeddings, compatible vector search and consumer tokenizer | Reranker, semantic extraction and generator |
| Semantic librarian | Core plus validated local auxiliary inference | Permanent model residency; remote service |
| Additional compression | Context compiler plus a selected compressor | Compression when it provides no measured benefit |
| Answer generation | External compatible consumer with its own model/runtime | Running that generator inside Mnemvia |

The deterministic profile can return evidence without a language model. Exact token-budgeted output still requires the specified consumer tokenizer. Missing components produce explicit capability/degradation information, not silent downloads or cloud calls.

A Markdown/Obsidian folder is an input source, not a dependency on the Obsidian application or plugin. CLI/API consumers may include assistants, editors and chat applications; no integration is currently implemented. A remote model consumer is possible by deliberate user choice, but the local engine must not require one.

## Admission and update rules

Before adoption, record purpose, category, upstream URL, version/commit, license/notices, transitive requirements, supported platforms, offline behavior, resource impact, security review and replacement/removal plan. Add a meaningful integration test and a maintainer responsible for updates.

Use the actual dependency manifest and lockfile as the authoritative installed set once code exists. Keep this register synchronized with those files and release notes. Generate an SBOM from the resolved build rather than relabeling this candidate list as an SBOM.

For models and datasets, additionally record provenance, hashes, tokenizer/template, redistribution terms and explicit provisioning. Changing an embedding fingerprint requires an isolated index generation and compatibility tests.

Mnemvia's Apache-2.0 license does not relicense upstream projects or weights. Review the exact selected revision and distribution under the [licensing policy](docs/LICENSING.md). No third-party code or weights are introduced by this documentation update.

## Downstream projects

No projects are currently confirmed to depend on Mnemvia. Maintain a future consumer list only from verified integrations and with accurate ownership/compatibility information. Do not imply adoption by projects listed above.

