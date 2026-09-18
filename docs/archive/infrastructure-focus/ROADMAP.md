# Opseld roadmap

Milestones describe acceptance criteria, not release dates. No milestone has been completed beyond preparing the initial design and community documents. Maintainer availability and evidence from earlier milestones determine scheduling.

## M0 — Public project foundation

- Choose the name and license; prepare contribution, governance and security policies.
- Establish the actual hosting owner, named maintainer roster and private reporting channel before a public alpha.
- Publish portable documentation after checking ownership and removing private material.
- Decide whether to proceed based on a real operator problem, available evaluation cases and maintenance capacity.

Current status: name/license selected and local documents prepared. Hosting, namespace reservation, publication and reporting-channel setup have not been performed.

## M1 — Offline evidence demo

- Define a small versioned evidence-bundle format with scope, timestamps, source references and expected outcomes.
- Include at least five entirely synthetic, redistributable incident scenarios, including insufficient evidence and misleading metadata.
- Provide deterministic checks that work without model weights.
- Add an optional local model path, simple text retrieval and explicit relationships for comparison.
- Compare against HolmesGPT where an equivalent restricted setup is feasible.
- Publish reproducible commands, environment details, limitations and measured results only after running them.
- Demonstrate the baseline from a clean checkout without production access or paid services.

Exit gate: reproducible evidence of incremental usefulness, or a documented decision to simplify. The demo is not a production release.

## M2 — Linux read-only alpha

- Homologate one Linux distribution/version and explicitly documented service scenarios.
- Use existing telemetry where practical; implement only missing bounded collection.
- Add scoped identity, authorization, secret handling, durable audit and agent revocation where an agent is used.
- Correct queue admission, evidence retention, current-permission checks and temporal evaluation leakage described in the critical review.
- Provide a minimal incident/evidence interface and a tested installation/removal procedure.
- Run security and diagnosis evaluation on held-out scenarios; report denominators and failures.
- Activate and test the private vulnerability reporting channel before release.

Exit gate: all applicable v0 security requirements, a working restore exercise and at least one independently reproduced installation. If no external tester is available, record the missing validation instead of claiming the gate passed.

## M3 — Operational beta

- Obtain feedback from multiple independent environments, targeting three where feasible.
- Add deployment identity and carefully selected dependency parsers.
- Introduce hybrid retrieval only if evaluation demonstrates a useful gain.
- Extend retention, recovery, upgrade/migration and operating documentation.
- Select Windows or Proxmox based on demonstrated demand and a committed maintainer; do not require both at once.

Exit gate: documented compatibility matrix, observed operational value and capacity to maintain supported integrations.

## M4 — Governed actions experiment

- Evaluate one bounded action for one service in a disposable or staging environment.
- Bind approval to target, parameters, policy version and current preconditions.
- Handle unknown outcomes, duplicate requests, reconciliation and recovery explicitly.
- Compare an existing automation executor with a minimal project-specific implementation.
- Keep unrestricted shell, destructive actions and automatic production remediation out of scope.

This milestone depends on proven identity, policy, evidence and recovery controls, not on completing every platform connector. A production action release needs a separate readiness review.

## Deferred until justified

Fine-tuning, semantic auto-merging, dedicated graph/vector servers, a hosted service and a general agent framework are not baseline commitments.

## Release policy

The first runnable demo may be tagged as a pre-release after its gate passes. Alpha/beta versions clearly state support boundaries. Version 1.0 requires stable public contracts, documented upgrade/recovery paths, reproducible evaluation and sufficient maintainership; a calendar date alone cannot trigger it.

During 0.x, breaking changes require release notes and a migration or explicit reset path. Security fixes take priority over new connectors. The initial maintenance policy supports only the latest published pre-release; there are currently no published versions.
