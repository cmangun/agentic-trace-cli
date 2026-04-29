---
name: Feature request
about: Propose new behavior or an enhancement
title: "[feature] "
labels: ["enhancement"]
---

## Problem

What user-visible problem does this solve, or what current behavior is inadequate?

## Proposed change

What should the component do differently? Be concrete about inputs, outputs, and error modes.

## ADR check

Does this change touch any of:

- [ ] Receipt schema, canonicalization, hashing, or signing semantics
- [ ] Hash-chain or bundle composition rules
- [ ] Policy-decision-receipt shape, decision semantics, or default posture
- [ ] Eval-harness scenario taxonomy or regression-gate semantics
- [ ] Any cross-component interop contract documented in [agentic-evidence/INTEROP.md](https://github.com/cmangun/agentic-evidence/blob/main/INTEROP.md)

If any box is checked, this proposal needs an ADR before it can merge. Link the draft ADR (or open one) in `adrs/NNNN-<slug>.md` per the local `adrs/0000-template.md` structure.

## Compatibility implications

Per [agentic-evidence/VERSIONING.md](https://github.com/cmangun/agentic-evidence/blob/main/VERSIONING.md):

- [ ] Backward compatible (no consumer changes required)
- [ ] Backward incompatible — breaking change requires VERSIONING.md §2 review and a major-version bump on this component

## Alternatives considered

What did you rule out and why? (Mirrors the ADR template's `## Alternatives Considered` section so the request transitions cleanly into an ADR if accepted.)
