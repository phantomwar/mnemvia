# Mnemvia governance

## Initial model

Mnemvia starts as a maintainer-led open-source project. The founding project owner appoints the initial maintainers. This repository does not yet identify a hosting organization or appointed maintainer accounts; publication must establish those identities transparently rather than invent a community or foundation.

Contributors retain copyright. No foundation affiliation, commercial sponsor or external maintainer is implied by this document.

## Roles

- Contributors propose issues, documentation, tests and implementation.
- Reviewers provide technical review without automatic merge or release authority.
- Maintainers own triage, releases, security response and decisions for their documented areas.
- A release owner coordinates each release and its evidence; the role does not override security gates.

The public roster must list actual accounts, responsibilities and security contacts before a public alpha. Until then, governance is a prepared policy, not an operating multi-person committee.

## Decisions

Routine changes can be accepted by the responsible maintainer after review. Significant architecture, schema, scope or dependency decisions use a written proposal describing alternatives, trade-offs and evaluation. Keep normal proposals open for at least seven calendar days for comment, except urgent security fixes.

Seek consensus. If consensus is unavailable, the founding maintainer decides during the initial phase and records the rationale and objections. Once there are at least three active maintainers, adopt majority decisions for disputed technical proposals, with conflicts of interest disclosed and at least two participating maintainers. This transition must be recorded publicly.

Security-critical changes should receive a second review when another qualified reviewer is available. A solo maintainer must disclose that limitation and cannot describe a self-review as independent review.

Changing a license for future work does not revoke grants for existing releases. Relicensing contributed work requires the necessary rights; DCO sign-off is not blanket permission for arbitrary relicensing.

## Becoming a maintainer

Consider sustained, constructive contributions, review quality, familiarity with the security model and willingness to maintain a defined area. An existing maintainer nominates a contributor publicly; allow at least seven days for feedback. A title is not awarded solely for donation volume, stars or number of commits.

Document inactivity or resignation and hand over access. Apply least privilege, multifactor authentication for hosting/release accounts and prompt removal of obsolete access. As capacity grows, aim for at least two release-capable maintainers; do not claim this redundancy exists today.

## Sustainability and independence

Sponsorship, grants, support and integration work may fund maintenance. Funding does not automatically buy merge rights or exclusive control over the roadmap. Disclose sponsored work and conflicts in relevant proposals.

The planned public distribution includes the controls required to run it safely: authorization, auditing, redaction and recovery are not reserved for a paid edition. Optional commercial services may coexist with the Apache-2.0 project. There is no hosted service or support SLA today.

## Project health

Track successful clean installations, reproduced evaluations, time to useful feedback, unresolved security work and active maintainers. Stars and download counts are secondary signals. If maintenance stops, clearly mark the project unmaintained and document transfer or archival options instead of silently implying support.


## Dependency decisions

Use the [dependency register](DEPENDENCIES.md) for adoption status and the PRD for product scope. Choosing a benchmark does not approve a production dependency. A build-versus-compose decision records ownership of lifecycle, evidence and context contracts, plus maintenance and replacement costs. Listing related projects does not imply endorsement or partnership.

