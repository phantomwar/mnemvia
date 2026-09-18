# Security policy

## Current status

Opseld is a design-stage repository with no runnable release and no supported production version. No dedicated private vulnerability reporting channel has been established yet.

Before publishing a public alpha, the hosting owner must enable and test private vulnerability reporting on the selected forge, or publish a monitored private security contact here. That is a release gate, not an already available service. Do not assume an unlisted email address exists or sends reports to the maintainers.

## Reporting

Public issues are suitable for non-sensitive design questions. Do not publish exploit details, credentials, production logs, personal data or findings that expose another organization.

When the private channel is established, use the reporting instructions added to this file. If a hosting platform already offers a verified private report control for this repository, it may be used; otherwise keep sensitive details private until a maintainer provides a verified channel. The project makes no response-time promise while no channel or response team exists.

A useful report identifies the affected version, impact, prerequisites and a minimal synthetic reproduction. Avoid including real secrets or customer evidence.

## Planned release security requirements

- Document the supported platform/version and known limitations.
- Check dependency and artifact provenance; retain required notices.
- Review tool permissions, scope isolation, redaction and evidence handling.
- Exercise restore, update and certificate/credential revocation where applicable.
- Publish checksums and build provenance appropriate to the release pipeline.
- Prevent untrusted pull-request workflows from accessing release secrets.
- Configure private reporting and identify the responsible maintainers.

For initial 0.x releases, the latest published pre-release is the intended supported line. Backports and response targets depend on demonstrated maintainer capacity and must be explicitly published. There are no supported releases today.
