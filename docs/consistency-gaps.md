# Consistency gaps

Internal consistency gaps found by the reviewer, recorded
plainly and evidence-first. Each entry quotes or closely
paraphrases both sides with file and section or module.
A gap is moved to resolved only when a later commit
changes one side so the conflict no longer holds, citing
that commit. A gap is not resolved by argument alone.

## 1. "Ending" means harness retirement in one place and run termination in others

- Area: theory versus adaptations (terminology).
- Side A — `docs/theory.md`, "Ending: when a harness may
  properly end, and how it ends well": "Ending a harness
  is not the same as halting a run. A run is halted by
  the overseer, or aborted on a liveness verdict, and
  the harness stands ready for the next run. Ending the
  harness is retiring the standing machinery itself."
- Side B — `harness-core/src/adoption.rs`, `AdoptedHarness`
  trait: "Its ending: when a run ends, and who decides."
  All four adapters follow the run sense in `ending()`:
  e.g. `adaptations/src/gemini_cli.rs` and
  `adaptations/src/opencode.rs` ("The loop ends when the
  task is complete, when the user stops it, or when a
  control refuses a required action"), and
  `adaptations/src/swe_agent.rs` ("The loop ends, done
  becomes true, on submit, on forfeit, on cost, context,
  or timeout, or by autosubmission after an error").
  The theory's own worked example uses the run sense
  too: `docs/theory.md`, "A worked example: SWE-agent",
  "Ending and verdict come apart here, and the split is
  worth keeping. The loop ends — `done` becomes true —
  on submit, on forfeit, on cost, context, or timeout".
- Why they cannot both hold as written: under Side A,
  a run halting or completing is explicitly not an
  ending, so a trait method and four adapters that call
  run termination "ending" use the term for what Side A
  says it is not. Under Side B's usage, the Ending
  section is about something else under the same name.
  A reader cannot tell from the word alone which sense
  is meant. (This is a terminology collision, not a
  claim that any membership result is wrong.)
- Date first noted: 2026-10-24.
- Status: open.

## 2. The adapter shape descriptions disagree on arbitration, and two adapters say nothing about it

- Area: theory versus adaptations (adapter specification).
- Side A — `docs/OKR.md`, Month 1 Objective 1: a stub
  adapter exists "naming, in the core's terms, X's loop,
  action space `A(h)`, state split, views, arbitration
  and controls, and ending."
- Side B — `adaptations/README.md`, adapter description:
  each adapter names "that harness's loop, action space,
  state split, views, controls, ending, and membership"
  — arbitration is missing from the list.
- Side C — `harness-core/src/adoption.rs`, `controls()`
  doc: "Its arbitration rule, if any, and its
  designer-fixed controls" — arbitration is conditional,
  not required.
- The adapters themselves: `adaptations/src/codex_cli.rs`
  states in `controls()` that escalation "is not
  arbitration in the theory's sense";
  `adaptations/src/swe_agent.rs` states "No arbitration"
  only in its `gaps()` list; `adaptations/src/gemini_cli.rs`
  and `adaptations/src/opencode.rs` contain no mention
  of arbitration at all (in `controls()` or `gaps()`).
- Why they cannot both hold as written: the three places
  that define what an adapter must say disagree —
  required (OKR), omitted (adaptations README),
  conditional (trait) — and for Gemini CLI and OpenCode
  a reader checking the OKR item cannot tell whether
  arbitration is absent, as in SWE-agent, or simply
  unnamed. Arbitration is a minimal-core claim
  (`docs/theory.md`, "The minimal core", fifth claim),
  so the silence is about a core item, not a detail.
  (This entry does not claim Gemini CLI or OpenCode
  lack arbitration; it records that the repo does not
  say, while its own specifications disagree on
  whether it must say.)
- Date first noted: 2026-10-24.
- Status: open.
