# AGENTS.md

Guidelines for working in this repo, whether you are a human or an agent.
Keep this file short. If a rule here stops being true, change the rule
instead of working around it.

## What this repo is for

One simple, unified, mathematical theory of what a harness is, stated
precisely enough to be implemented in code, with a core so tight and
generalizable that any harness can build on top of it.

A harness is the designer-controlled machinery between a policy and the
world: it decides what the policy sees next, what its output does, when
a run stops, and who can alter or suspend it.

This repo is not a harness. It does not run one. It defines one, tests
the definition against real harnesses in stubs, and keeps only what
survives that test.

## The core test

An addition belongs here only if it passes all three:

1. It changes an adoption or a membership result. If dropping it leaves
   every loop, action set, state split, observation function, control,
   termination, and membership result unchanged, it does not belong.
2. It can be stated simply and defined mathematically, in standard
   terms.
3. It can be implemented as a stub that other harnesses can build on.

Interesting is not enough. Governance, goodness, and operational detail
fail this test unless they change an adoption or a membership result.

## Theory writing

- Use normal mathematical, computer-science, and software-engineering
  language. Do not invent a term when a standard term exists. A new
  term needs a sentence saying exactly why the standard term fails.
- Delete a sentence that is true by definition alone. A definition is
  kept once, in standard terms, and only if it is then used to classify
  a real harness, state a checkable condition, or give a counterexample.
- Every rewrite in normal language goes through a Simplified Technical
  English filter: short sentences, active voice, simple tenses, one
  term for one concept, no contractions, no semicolons in prose.
  Mathematical symbols and expressions stay verbatim, and every symbol
  is defined before its first use, stating what it stands for.
- Basic questions require consideration, not resolution: how the
  question bears on the theory, what the theory takes for granted, and
  where it leaves the question open. Do not mark a question answered
  by assertion.
- Do not re-add material that was removed for failing the core test
  unless it now passes that test. That includes, as examples:
  succession, retirement, outstanding obligations, chains of
  succession, identity across revision, trust records and trust
  anchors, liveness and progress over histories, rules governing
  design decisions, human intervention in full, error models in full,
  cost and latency in full, and verdicts by harness kind.

## Code

- `harness-core/` and `adaptations/` are stubs only. No behaviour, and
  no tests, until a later decision explicitly says otherwise.
- Every theory claim in the core has a stub counterpart, and every
  stub has a theory counterpart. If the two drift apart, fix the drift
  or remove one side. Do not leave a stub with no theory claim, or a
  theory claim with no stub.
- No comments or docstrings in code. Names must be clear on their own.
  If a name is not clear without a comment, rename it.
- After changing a stub, run a check on both crates, and regenerate
  the visualization from the repo root. Do not edit
  `visualization/index.html` by hand.

## Adapters

- Adapters cover open-source harnesses only, and only the harness's
  own code, not a model service it talks to by default.
- An adapter names, in the core's terms: policy, loop, action space
  `A(h)`, state split, observation functions, controls, termination,
  and membership result.
- State the precedence position explicitly in `controls()`: the rule
  itself, that there is none, or that it is not yet stated. Silence
  is not a statement.
- List gaps plainly where the fit is uncertain or unverified. A gap
  is a finding, not a failure, and it is not a reason to force a fit.
  Do not treat an adapter description as a claim about a real harness
  until it has been checked against that harness's code or docs.

## Verification and commits

- Check a claim against the actual files, not from memory. Quote or
  closely paraphrase both sides when reporting an inconsistency.
- If you find an internal inconsistency, record it evidence-first in
  `docs/consistency-gaps.md`, with title, area, conflicting
  statements and their locations, why they cannot both hold, date
  first noted, and status open. Do not resolve a gap by argument
  alone; resolve it only when a later change makes the conflict
  no longer hold, citing that change.
- Keep changes focused and reversible, with Conventional Commits
  messages. Do not delete this repo, and do not publish its contents
  elsewhere.
- Keep personal information out of files, commits, and reports.
