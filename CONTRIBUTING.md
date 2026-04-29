# Contributing

This component is part of the [Agentic Evidence Suite](https://github.com/cmangun/agentic-evidence). Read the [reference architecture](https://github.com/cmangun/agentic-evidence/blob/main/REFERENCE-ARCHITECTURE.md) before proposing changes that touch suite-level contracts.

## Workflow

1. Open an issue first for anything beyond a typo, README touch-up, or test-only change. Use the issue templates in `.github/ISSUE_TEMPLATE/`.
2. Fork the repo and create a topic branch off `main`.
3. Make focused commits — one logical change per commit. Commit messages follow `[phase|component] subject`, lowercase verb-leading, ≤ 72 chars on the subject line. Examples: `[receipts] add Unicode NFC vector`, `[fix] ed25519 verifier rejects empty key`. The leading bracket label classifies the change; the body explains the why.
4. Open a PR using the template in `.github/PULL_REQUEST_TEMPLATE.md`. Fill every section.
5. CI must be green before review. CI green is required, not aspirational — a red build is a hard merge gate. If a flake-prone test fails, fix the test rather than re-running until green.

## When an ADR is required

A change requires an ADR (Architecture Decision Record) under `adrs/NNNN-<slug>.md` before it can merge if it touches **any** of:

- Receipt schema, canonicalization (JCS RFC 8785), hashing, or signing semantics.
- Hash-chain construction, prev-hash linkage, or genesis-receipt convention.
- Bundle composition, root-hash construction, or manifest signing rules.
- Policy-decision-receipt shape, decision semantics, or default posture.
- Eval-harness scenario taxonomy, regression-gate semantics, or the boolean per-scenario contract.
- Any cross-component interop contract documented in [INTEROP.md](https://github.com/cmangun/agentic-evidence/blob/main/INTEROP.md).

Use the local `adrs/0000-template.md` and follow the 5-section structure (Status / Context / Decision / Alternatives Considered / Consequences). Name at least two specific alternatives in the alternatives section — generic "we could have done it differently" placeholders block merge.

## Compatibility

Read [VERSIONING.md](https://github.com/cmangun/agentic-evidence/blob/main/VERSIONING.md) before changing any externally-visible contract. The PR template's breaking-change checkbox triggers a VERSIONING.md §2 review for breaking changes.

## Identity

Use your own author + committer identity on commits. Bot or proxy attribution that masks the human contributor will be rejected at review. Sign-off (`Signed-off-by:` trailer) is not currently required but is welcomed.

## License

By contributing, you agree your contribution is licensed under the repo's [LICENSE](LICENSE) (MIT) on the same terms as the rest of the codebase.
