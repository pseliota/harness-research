# adaptations — stubs only

A Rust crate parallel to `harness-core`, dedicated to
demonstrating how the core can be adapted to various
popular harnesses. Each module is one stub adapter: a
struct implementing `harness_core::adoption::AdoptedHarness`,
naming, in the core's terms, that harness's loop, action
space, state split, views, controls, ending, and
membership.

Nothing here runs a harness, and there are no tests.
The strings returned by an adapter are names and
descriptions in the core's terms, not behaviour. They
are stubs to be checked against the real harness's code
and documentation before they are treated as claims
about it. Each adapter therefore also lists `gaps`:
places where the fit is uncertain, or where a detail
still needs verifying.

Adapters included:

- `claude_code` — Claude Code
- `codex` — OpenAI Codex
- `cursor` — Cursor, agent part only (the editor itself
  is a neighbour, not a harness)
- `gemini_cli` — Gemini CLI
- `copilot` — GitHub Copilot, agent part only
- `opencode` — OpenCode
- `swe_agent` — SWE-agent, moved here from
  `harness-core/src/adoption.rs` so the core stays
  free of any one harness; it is the most worked
  example, checked against SWE-agent's code and
  default configuration, not only its paper

To add a new adapter: copy one module, implement
`AdoptedHarness` for the new harness, list its gaps,
add the module to `src/lib.rs`, and add it to the list
above. Do not add behaviour, and do not add tests,
until a later decision explicitly says otherwise.
