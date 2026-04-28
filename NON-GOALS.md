# Non-goals

What `agentic-trace-cli` deliberately does not try to be. The CLI is the developer surface; companion components own the surfaces it does not.

## Not a server or daemon

The CLI is stateless and one-shot. Each invocation operates on a single bundle, exits, and leaves no background process. Adopters wanting long-running services around bundle handling build them on top of the CLI's commands rather than expecting the CLI to be the service itself.

## Not a UI

Visual inspection of bundles — timeline view, redaction display, policy decision rendering — belongs to `agentic-evidence-viewer`. The CLI emits exit codes and JSON output for engineers and CI systems; the viewer is the auditor's surface for browsing.

## Not a policy engine

The CLI verifies signatures, hash chains, and schema conformance. It does not evaluate policy. Policy evaluation — the gate decision — happens at runtime in `agentic-policy-engine` and is recorded in decision receipts that the CLI then verifies as data.

## Not a multi-bundle manager

Each invocation operates on one bundle. The CLI does not maintain a registry, search across bundles, deduplicate across bundles, or coordinate operations spanning multiple bundles. Adopters needing multi-bundle workflows compose CLI invocations in their own scripting layer.

## Not authoring synthesis

The CLI verifies, transforms (sign, redact, export), and emits — it does not generate receipts from non-receipt input. Receipt authoring happens in the agent runtime as actions execute; the CLI operates after the fact on already-emitted receipts.
