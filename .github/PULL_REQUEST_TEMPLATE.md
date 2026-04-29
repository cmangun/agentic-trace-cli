<!--
Pull-request template for components of the Agentic Evidence Suite.
Fill every section. Empty sections delay review.
-->

## What

One-line summary of the change.

## Why

What user-visible problem does this solve, or what invariant does it preserve? Link the related issue if applicable.

## Test evidence

How was this verified? At minimum:

- [ ] Existing test suite still passes (CI green is required, not optional)
- [ ] New tests added for any new behavior or for regression-locking a fix
- [ ] Conformance vectors updated (for changes touching `agentic-receipts/vectors/`)
- [ ] Manual reproduction steps included if the test surface cannot fully cover the change

## ADR link

Does this change touch receipt semantics, signing, canonicalization, hash-chain composition, policy-decision shape, eval-harness gate semantics, or any documented cross-component interop contract?

- [ ] No — proceed
- [ ] Yes — link the ADR: `adrs/NNNN-<slug>.md` (or the meta-repo ADR if the change is suite-wide)

PRs that touch any of those surfaces without a corresponding ADR will not be merged.

## Breaking change

- [ ] **Backward compatible** — no consumer changes required.
- [ ] **Breaking change** — this changes a contract per [agentic-evidence/VERSIONING.md §2](https://github.com/cmangun/agentic-evidence/blob/main/VERSIONING.md#2-breaking-changes). VERSIONING.md §2 review is required before merge, and the next release of this component must be a major-version bump (or follow the §3 deprecation window, whichever applies).

If breaking, list the consumers known to be affected and the deprecation path (if any) being offered.

## Checklist

- [ ] Commit messages follow the repo convention `[phase|component] subject`
- [ ] Author + committer identity is the contributor's own (no anonymous co-author trailers from tooling)
- [ ] CI is green
