# Consistency gaps

Internal consistency gaps found by the reviewer, recorded
plainly and evidence-first. Each entry quotes or closely
paraphrases both sides with file and section or module.
A gap is moved to resolved only when a later commit
changes one side so the conflict no longer holds, citing
that commit. A gap is not resolved by argument alone.

Note: an earlier version of this file was deleted by
c6d1672 ("cut repo to core goal only"), which states
its entries referenced removed sections. Its three
entries were all already marked resolved. The entries
below are new, against the post-cut theory.

## 1. Open question 2 uses `G`, which the theory never defines

- Area: within `docs/theory.md` (definitions versus open questions).
- Side A — `docs/theory.md`, "Definitions": defines
  s, w, h, a, o, pi, p, s', V, A, A1, A(h), principal,
  and p_i(o_i | s). No definition of `G` appears there
  or anywhere else in the current repo (outside
  `target/`): the only occurrence of `G` is Side B.
- Side B — `docs/theory.md`, "Open questions", 2:
  "What distinguishes a good harness from a merely
  working one, once cost, robustness, legibility to
  humans, and failure modes are included, not just
  reaching `G`?"
- Why they cannot both hold as written: the question's
  contrast turns on what reaching `G` means, and the
  document gives the reader no referent for `G`. A
  reader cannot tell from the current text alone
  whether `G` means a goal set, a goal state, or
  something else; the pre-cut meaning survives only
  in git history, not in the document.
- Date first noted: 2026-10-24.
- Status: resolved by ddd4287 ("docs(theory): remove dangling references to undefined G and error model"), which replaced `G` with "the task goal".

## 2. The stub's closed principal type cannot express the designer and subagent observations the POMDP section describes

- Area: theory versus `harness-core/` (types).
- Side A — `docs/theory.md`, "POMDP as the shape of
  one step", point 2: "Each principal has its own
  p(o | s) over the same s. The user, the designer,
  and a subagent see different observations of the
  same state." "Definitions" defines a principal as
  "a party with its own goal, observation function,
  and write permission set, such as the user, the
  agent, or the overseer" — an open list ("such as").
- Side B — `harness-core/src/observation_functions.rs`:
  `Principal` is a closed enum with exactly three
  variants, `Agent`, `User`, `Overseer`, and
  `ObservationFunction::observe` takes that enum, so
  a designer or subagent observation function cannot
  be named in the core's terms. The four adapters
  likewise list only Agent, User, and Overseer
  observation functions.
- Why they cannot both hold as written: if the
  designer and the subagent are observation bearers
  in the theory's sense, the core type excludes two
  of them; if they are not principals, the POMDP
  sentence attributes principal-style observations
  to non-principals. One side needs narrowing: either
  the enum is illustrative rather than exhaustive, or
  the POMDP sentence should not present the designer
  and the subagent as observation bearers.
- Date first noted: 2026-10-24.
- Status: resolved by 466abb9 ("docs(theory): name agent, not designer and subagent, as observation bearers"), which narrowed the POMDP sentence to the user, the agent, and the overseer, the three variants of the stub enum. A subagent's observations are the inner loop's agent observations, and the designer fixes observation functions rather than bearing one.

## 3. The action section locates a rule "in the error model", which the theory does not contain

- Area: within `docs/theory.md` (action section versus basic questions).
- Side A — `docs/theory.md`, "Action space at levels
  of abstraction": "Unparseable token sequences are
  no action at all, not failed actions. The harness
  still responds, with an error, a retry, or a halt.
  That response follows a transition rule in the
  error model, not a rule in the action space."
- Side B — the theory contains no error-model section
  and no definition of an error model. The only other
  mention is `docs/theory.md`, "Basic questions", 8:
  "What happens when the model produces malformed
  output, refuses, loops, or calls a tool that fails
  or times out? What is the harness's error model?"
  — posed as a question, not given as a model.
- Why they cannot both hold as written: Side A points
  the reader to a rule located in "the error model",
  but there is no error model in the document to
  locate it in, so the rule the sentence invokes
  cannot be checked against anything the theory
  states. (This is a dangling reference, not a claim
  that any membership result is wrong.)
- Date first noted: 2026-10-24.
- Status: resolved by ddd4287 ("docs(theory): remove dangling references to undefined G and error model"), which changed the sentence to "That response follows a transition rule, not a rule in the action space", removing the reference to an error model.
