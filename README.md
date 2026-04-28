# agentic-trace-cli

> Part of the [Agentic Evidence Suite](https://github.com/cmangun/agentic-evidence) — six interoperating components for verifiable agentic AI. See [`REFERENCE-ARCHITECTURE.md`](https://github.com/cmangun/agentic-evidence/blob/main/REFERENCE-ARCHITECTURE.md) for the suite-level architecture.

CLI tooling to create, sign, redact, verify, and export **verifiable agent traces**.

Implements:
- Bundle initialization and event appending
- Hash chaining and receipt emission
- Signature and verification
- Redaction with integrity preservation
- Evidence export (JSON + HTML)

Schema/spec source of truth: [agentic-receipts](https://github.com/cmangun/agentic-receipts)

## Commands (MVP)

| Command | Description |
|---------|-------------|
| \`agentic-trace init\` | Create a new bundle directory |
| \`agentic-trace append\` | Add an event to the trace |
| \`agentic-trace sign\` | Sign all receipts in a bundle |
| \`agentic-trace verify\` | Verify chain integrity + signatures |
| \`agentic-trace redact\` | Redact sensitive fields |
| \`agentic-trace export\` | Export evidence pack (JSON + HTML) |

## Quick Start

\`\`\`bash
agentic-trace init ./demo-bundle
agentic-trace append ./demo-bundle --event fixtures/event.json
agentic-trace sign ./demo-bundle --key ./keys/dev.key
agentic-trace verify ./demo-bundle
\`\`\`

## Suite

This repo is part of the **Agentic Evidence Suite**:
- [agentic-receipts](https://github.com/cmangun/agentic-receipts) (standard)
- [agentic-trace-cli](https://github.com/cmangun/agentic-trace-cli) (tooling)
- [agentic-artifacts](https://github.com/cmangun/agentic-artifacts) (outputs)
- [agentic-policy-engine](https://github.com/cmangun/agentic-policy-engine) (governance)
- [agentic-eval-harness](https://github.com/cmangun/agentic-eval-harness) (scenarios)
- [agentic-evidence-viewer](https://github.com/cmangun/agentic-evidence-viewer) (review UI)

## License

MIT