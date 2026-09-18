# Contributing to Mnemvia

Mnemvia has a deterministic M0 CLI. Useful first contributions include a focused documentation improvement, a synthetic knowledge corpus proposal, an evaluation design or a tested improvement to the lexical profile. Run `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` for Rust changes. The public repository runs formatting, check, Clippy and test jobs on pushes and pull requests; there is no published release yet.

## Contribution workflow

1. For a significant feature, describe the user problem, proposed scope, alternatives and acceptance evidence in an issue before implementation.
2. Keep pull requests small and identify the requirement or issue they address.
3. State what changed, why, how it was checked and any remaining limitation. Never claim tests ran if they did not.
4. Include migration and compatibility notes for schema or protocol changes.
5. Add tests that demonstrate meaningful behavior or a security boundary when implementation exists.

English is preferred for public interfaces and canonical documentation. Portuguese and English are both welcome in discussions; language should not block a useful contribution.

## Origin and licensing

Contributions are accepted under Apache-2.0 and the [Developer Certificate of Origin 1.1](DCO.txt). Contributors retain copyright in their work. No copyright assignment or separate CLA is required by the initial project policy.

Every contribution commit must include a `Signed-off-by: Name <email>` trailer identifying the contributor and certifying the DCO. Configure an appropriate public contribution email before creating public commits; never invent an identity or sign on another person's behalf. DCO sign-off is an origin certification, not a cryptographic commit signature.

By submitting work, you must have the right to contribute it under these terms, including any employer authorization that applies. Preserve third-party notices and disclose imported material with its origin and license. Code suggested by AI is subject to the same review, origin and licensing requirements. The human contributor remains responsible for understanding and validating it.

DCO sign-off is required by policy. Automated DCO enforcement is still a release gate to configure before the first code release.

## Knowledge fixtures and privacy

Public fixtures must be synthetic and created for redistribution, or have documented permission and compatible licensing. Sanitizing production logs does not automatically make them safe or redistributable.

Do not submit production credentials, customer records, real host inventories, private repository content, model weights or unlicensed copied runbooks. Keep fixture provenance and expected outcomes alongside the fixture. Keep evaluation answers and later source revisions separate from retrieval inputs to prevent leakage.

## Dependency proposals

Explain the capability, license, maintenance status, platform support, offline behavior and resource impact. Prefer a small adapter to introducing a mandatory service for one optional feature. Optional integrations still need explicit license and security review.

## Reviews and decisions

Routine fixes follow maintainer review. Architecture, security boundaries, public schemas and licensing changes need a written proposal and follow [GOVERNANCE.md](GOVERNANCE.md). An open issue is not a promise of implementation or support.

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for participation expectations and [SECURITY.md](SECURITY.md) for sensitive reports.


## Keeping dependencies and documentation accurate

Use [DEPENDENCIES.md](DEPENDENCIES.md) to distinguish adopted components, candidates, evaluation tools and research references. Update it with the manifest/lockfile change that adopts a component. Record versions, transitive requirements, model provenance and replacement strategy. Research citations are not installation requirements.

Keep both READMEs aligned with the PRD and implementation status. Do not advertise unverified integrations, platform support or results. Superseded proposals belong in the historical section.

