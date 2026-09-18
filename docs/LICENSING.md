# Licensing policy

Decision date: 2026-09-18. Project: Mnemvia. Selected license: **Apache-2.0**.

## Why this license

The initial objective is broad adoption, integration and contribution, including commercial environments. Apache-2.0 combines permissive redistribution with explicit contributor patent terms and contribution rules. It does not require proprietary derivatives to publish their changes, and it does not grant trademark rights. These are deliberate trade-offs, not accidental omissions. [License text](https://www.apache.org/licenses/LICENSE-2.0.html).

| Candidate | Main distinction | Decision for Mnemvia |
|---|---|---|
| Apache-2.0 | Permissive, with express patent and contribution provisions | Selected for integration and adoption |
| MIT | Short permissive license; lacks a separate express patent clause in its text | Valid alternative, but less explicit for this choice |
| MPL-2.0 | File-level copyleft for covered source when distributed | Alternative if reciprocity for modified core files becomes the primary objective |
| AGPL-3.0 | Strong copyleft with a source-offer requirement for users interacting remotely with a modified program | Alternative if reciprocity for hosted modifications becomes the primary objective |

Sources: [MIT](https://opensource.org/license/mit), [MPL FAQ](https://www.mozilla.org/en-US/MPL/2.0/FAQ/), [AGPL text, including section 13](https://opensource.org/license/agpl-3.0). These summaries do not replace the licenses or determine the status of a particular combined work.

Open source does not mean noncommercial. The project will not add a noncommercial, anti-cloud or field-of-use restriction to Apache-2.0 while describing the result as the same OSI-approved license. [Open Source Definition](https://opensource.org/osd).

## Covered material

The repository's original code, documentation, examples and synthetic test fixtures use Apache-2.0 unless an explicit file-level exception states otherwise. Original contributors retain their applicable rights. Existing third-party notices are preserved.

Model weights, imported datasets, private source documents, external reports and third-party components are not relicensed by placing a reference to them in this repository. Record the license and source of every redistributed component and review the actual combination, rather than assuming separate processes automatically remove obligations.

No model weights are bundled in the initial repository. Future download helpers must disclose provenance, version/hash, license and user acceptance requirements where applicable. A runtime's license is not a model's license. Do not call all supported weights “open source” merely because they can be downloaded.

## Contributions

The project uses the [DCO 1.1](../DCO.txt), with incoming contributions under the same project license. DCO certifies contribution origin and the right to submit; it is not a copyright assignment, a separate license or blanket consent to future relicensing. [DCO source](https://developercertificate.org/).

## Notices and release review

Keep the canonical license in the root [LICENSE](../LICENSE). Add accurate SPDX identifiers to original source files as they are created, without inventing copyright owners. Do not alter third-party headers.

There is no project NOTICE file at this stage because no additional original attribution notice has been established. When incorporating material that requires preservation of notices, include those notices in the appropriate distribution. Do not put fictional sponsors, authors or affiliations in NOTICE. [Apache application guidance](https://www.apache.org/legal/apply-license).

Before each binary/container release, review the resolved dependency set, accompanying notices, bundled assets and model/data exclusions. Produce a dependency inventory/SBOM when the build exists. No automated license checker or release pipeline is configured yet.

## Name and marks

Mnemvia is the selected project name, not a registered-mark claim. Forks may describe their origin accurately but must not imply endorsement. No domain, trademark or package namespace has been registered by this planning work. Name screening and its limits are recorded in the [open-source plan](05-plano-open-source.md).


## Dependency inventory

[DEPENDENCIES.md](../DEPENDENCIES.md) is the candidate and relationship register. It is not an installed dependency list or SBOM. Adoption must record the exact upstream revision, applicable terms and required notices; future manifests/lockfiles and build-derived inventories identify what is actually distributed. Research references and potential downstream consumers must not be represented as bundled components.

