# Opseld

**Local infrastructure diagnostics, backed by evidence.**

[Português](README.pt-BR.md) · [Roadmap](ROADMAP.md) · [Contributing](CONTRIBUTING.md) · [Governance](GOVERNANCE.md) · [License](LICENSE)

Opseld is an open-source project being designed to connect operational knowledge, infrastructure observations and local language models. Its purpose is to help operators investigate incidents with traceable evidence, explicit uncertainty and bounded access to tools.

**Status: design stage. There is no runnable application or published release yet.** The repository currently contains product plans and community policies. The features below are planned, not implemented.

## First useful release

The first milestone is an offline investigation demo using synthetic incident bundles. It will compare deterministic checks, simple retrieval and evidence-linked context on the same cases. No production access or paid API will be required for the baseline demo; model-assisted runs will require separately provisioned, compatible weights.

The next milestone adds a narrowly scoped Linux read-only pilot. Windows, Proxmox, advanced knowledge retrieval and governed actions are later, independent milestones.

## Design principles

- Sources and observations remain distinguishable from inferred explanations.
- Every diagnostic claim links to evidence or is explicitly marked as unknown.
- Local inference is replaceable; rules and evidence remain usable without a model.
- Tool authorization is enforced by software, independently of the model.
- Secrets, customer incident data and model weights do not belong in the public repository.
- Existing collectors and integrations are evaluated before building replacements.
- No unrestricted shell tool, automatic destructive actions or mandatory cloud account.

Rust and SQLite are the initial implementation candidates. The prototype will validate their scope before the project commits to a broad platform.

## Get involved now

Useful contributions at this stage include reviewing the design, proposing synthetic incident cases, identifying incorrect assumptions and designing reproducible evaluation protocols. There is no build or installation command yet. Contribution and release checks will be introduced with their corresponding implementation, not advertised as already passing.

Use the repository host's issues and pull requests once the project is published. Do not submit credentials, production logs or personal information. See [security reporting status](SECURITY.md) before reporting sensitive findings.

## Documentation

- [Open-source strategy and license decision — Portuguese](docs/05-plano-open-source.md): current distribution, governance and positioning decisions.
- [PRD](docs/02-prd.md): functional requirements with the open-source amendment.
- [Execution plan](docs/03-plano-de-execucao.md): earlier staffed-team estimate, retained as a scenario rather than a release commitment.
- [Critical review](docs/04-revisao-critica-e-recomendacoes.md): findings and validation experiment.

README, contribution policies and roadmap use English for broader participation. Existing planning documents are in Portuguese; translations can be contributed. Issues in Portuguese or English are welcome.

## License and scope

Original project code, documentation, examples and synthetic fixtures are licensed under [Apache License 2.0](LICENSE), unless a file explicitly identifies different terms. Third-party components and model weights keep their own licenses. The repository license does not cover private input data or imply that Opseld is affiliated with the Apache Software Foundation.

Commercial use and proprietary derivatives are allowed under the license's conditions. Source publication is not required merely because a modified service is hosted. No trademark rights are granted by the software license. See [licensing policy](docs/LICENSING.md).
