# Security Policy

This component is part of the [Agentic Evidence Suite](https://github.com/cmangun/agentic-evidence). Suite-level threat-modeling lives at [agentic-receipts/spec/threat-model.md](https://github.com/cmangun/agentic-receipts/blob/main/spec/threat-model.md); per-component scope below.

## Supported versions

| Version line | Status |
|---|---|
| **v0.1** (current) | Actively patched. |
| pre-v0.1 (tagged-as-experimental, untagged) | Unsupported. Upgrade to a v0.1.x release. |

Per [VERSIONING.md](https://github.com/cmangun/agentic-evidence/blob/main/VERSIONING.md), v0.1 is the current public stability line. v1.0 will introduce a longer support window with a published EOL schedule.

## Reporting a vulnerability

Email **cmangun@gmail.com** with the subject line `[security] <component> <one-line summary>`. Do not open a public issue — the issue tracker is for non-sensitive defects only.

Please include:

- A description of the issue and the integrity property it threatens (chain integrity, signature verification, redaction integrity, deny-by-default posture, schema validation, or another).
- Reproduction steps or a minimal proof-of-concept.
- Affected version(s), commit SHAs, or release tags.
- Your name (or pseudonym) for credit, and whether you want public credit.

A dedicated security email alias and PGP/Ed25519 fingerprint will be published before v1.0; until then, the address above is the canonical channel.

## Coordinated disclosure window

The default coordinated-disclosure window is **90 days** from acknowledged report. This window aligns with the deprecation cadence in [VERSIONING.md §3](https://github.com/cmangun/agentic-evidence/blob/main/VERSIONING.md). The maintainer may request an extension if a coordinated multi-component fix is needed; reporters may shorten if active exploitation is observed.

A typical timeline:

- **Day 0** — report received; acknowledgment sent within 5 business days.
- **Day 0–30** — reproduction, severity classification, fix design.
- **Day 30–75** — fix implementation, regression test, conformance-vector update if applicable.
- **Day 75–90** — coordinated release, advisory publication, public CVE/GHSA filing.

## Out of scope

- Vulnerabilities in unsupported version lines (pre-v0.1).
- Issues in archived repos or private throwaway prototypes outside the suite.
- Behavior that is documented as a non-goal in the component's `NON-GOALS.md`.
