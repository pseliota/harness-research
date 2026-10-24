# adaptations — stubs only

A Rust crate parallel to `harness-core`, dedicated to
demonstrating how the core can be adapted to various
popular harnesses. Each module is one stub adapter: a
struct implementing `harness_core::adoption::AdoptedHarness`,
naming, in the core's terms, that harness's loop, action
space, state split, observation functions, precedence and controls,
termination, and membership. Every adapter states its
precedence position in `controls()`: the rule itself,
or that there is none, or that it is not yet stated.
Silence about precedence is not a statement — a reader
must be able to tell absence from an unnamed rule.

Nothing here runs a harness, and there are no tests.
The strings returned by an adapter are names and
descriptions in the core's terms, not behaviour. They
are stubs to be checked against the real harness's code
and documentation before they are treated as claims
about it. Each adapter therefore also lists `gaps`:
places where the fit is uncertain, or where a detail
still needs verifying.

Only open-source harnesses belong here. An adapter
covers the harness's own code, not the model service
it talks to by default; that service, where there is
one, has its own terms and billing, and is not part
of the adapter.

Adapters included:

- `codex_cli` — OpenAI's Codex CLI only, Apache 2.0.
  Codex's IDE extension and cloud service are not
  open source, so they are out of scope here. Its
  sandbox policy, approval modes, and escalation
  are verified in outline against openai/codex;
  escalation is a guardian-or-user decision with a
  turn or session scope, not precedence in the
  theory's sense, and is kept apart from the
  durable exec-policy amendment a user approval
  may also make. The rest of the adapter is still
  a summary, listed in its gaps.
- `gemini_cli` — Gemini CLI, Apache 2.0
- `opencode` — OpenCode, MIT
- `swe_agent` — SWE-agent, MIT, moved here from
  `harness-core/src/adoption.rs` so the core stays
  free of any one harness; it is the most worked
  example, checked against SWE-agent's code and
  default configuration, not only its paper

Removed, because they are not open source:

- Claude Code — proprietary, "All rights reserved",
  use under Anthropic's Commercial Terms, with
  modification and redistribution not granted.
- Cursor — commercial and proprietary; its public
  repository holds issue templates, not source.
- GitHub Copilot — the service is proprietary, and
  the Copilot CLI licence does not allow
  modification or derivative works.

Do not re-add a removed harness, or a closed part
of a kept one, without first checking its licence
and terms again.

To add a new adapter: copy one module, implement
`AdoptedHarness` for the new harness, state its
precedence position explicitly in `controls()` (the
rule, that there is none, or that it is not yet
stated), list its gaps, add the module to `src/lib.rs`,
and add it to the list above. Do not add behaviour, and do not add tests,
until a later decision explicitly says otherwise.
