# Objectives and key results

## Long-term direction

One simple, unified, mathematical theory of what a harness
is, easily implemented in code, with a core so tight and
generalizable that any harness can build on top of it.

That direction is not a monthly target. The monthly
objectives below break it down into reachable pieces.
A month does not have to end with a finalized theory,
with harnesses "solved", or with a specific decision
forced. A month succeeds if the work named below is
done honestly, even if the honest result is "the core
does not fit this harness", "the stubs and the theory
have drifted", or "no decision yet".

Key results are checkable outcomes, not quotas: a key
result is met when a reader can check it from the
theory, the examples, or the code, not when a count of
notes, commits, or refactors reaches a number.

## Month 1: adopt one harness, align stubs and theory, clean the theory

### Objective 1: Unify one popular harness under the theory, in stubs

Pick one popular harness, call it X, and show, in
stubs, how X can be adopted on top of the core in
`harness-core`.

- A stub adapter for X exists, naming, in the core's
  terms, X's loop, action space `A(h)`, state split,
  views, arbitration and controls, and run termination.
- The membership test is applied to X in stubs, with
  the result stated plainly: which of the four
  conditions hold, and which do not.
- Gaps are recorded where X does not fit the core,
  without forcing the fit. A gap is a finding, not a
  failure of the month.

### Objective 2: Ensure the stubs and the theory are properly aligned

- Every claim in the theory's minimal core has a
  counterpart in `harness-core`, and every stub in
  `harness-core` has a counterpart in the theory.
- Any mismatch found is listed, with one of three
  outcomes for each: change the stub, change the
  theory, or mark it as an open question.
- The package stays stubs only: no behaviour beyond
  `todo!()` or trait signatures, and no tests, until
  a later month explicitly decides otherwise.

### Objective 3: Make the theory, as it currently exists, clean and minimal

- The theory reads as a single pass. Governance topics
  — succession, ending, debts, identity, trust, roots —
  are clearly marked as applications or extensions,
  not as core, in both the theory and the package.
- One pruning pass is done: ideas that no longer hold
  are removed, or explicitly marked as open or
  deferred, rather than left to accumulate.
- The basic questions stay visible as questions. A
  basic question does not need a resolved answer;
  it needs explicit consideration: what the theory
  says about it, what it takes for granted, and
  where it leaves the question open.

## Later months: examples, not commitments

Later months may, for example:

- Adopt a second harness, from a different category
  than the first, and compare where the two adoptions
  agree and where they diverge.
- Consider one basic question in depth, from a
  worked example, rather than by argument alone,
  without requiring it to be resolved.
- Implement one core module, once stubs and theory
  are aligned and a month explicitly decides that
  implementation, with tests, is the next step.

None of these is a target for Month 1, and none
should be pulled forward just to make a month look
more ambitious.
