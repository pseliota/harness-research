# Objectives and key results

High-level objectives for this research. The key results are
checkable outcomes, not quotas: a key result is met when a
reader can check it from the theory, the examples, or the
code, not when a count of notes, commits, or refactors
reaches a number.

## Objective 1: One simple, unified, mathematical theory of what a harness is

- The core is stated in a small set of definitions, each
  necessary: removing one loses something the others
  cannot recover.
- The basic questions are answered, or explicitly left
  open with what would settle them: the action space,
  whether a POMDP is the right framework, steps and
  ending, state versus context and memory, tools versus
  subagents and skills, harness versus framework,
  errors, costs and permissions, human intervention,
  and variation across kinds of harnesses.
- The theory can say, for a candidate, whether it is a
  harness or a neighbour, and why.

## Objective 2: A theory grounded in the harnesses used today

- The major kinds of harnesses are surveyed as worked
  examples, not just named: coding agents, frameworks
  for building agents, and research prototypes.
- Each worked example can be expressed in the core's
  terms — its loop, action space, state, views,
  arbitration, and ending — without special pleading.
- Counterexamples and failures are recorded where the
  core does not fit, and the core is revised, or its
  scope narrowed, in response.

## Objective 3: A core that is easily implemented, and that any harness can build on

- Each core definition maps to code: a data structure,
  a function, or an interface, so a reader can see
  the implementation shape of the theory.
- A harness is built on the core, or an existing
  harness is adapted to it, without changing the core.
- New developments fit as extensions outside the core,
  not as revisions to it, unless a test shows that a
  core claim itself fails.

## Objective 4: A theory that stays tight and readable as it grows

- Elaborate topics — succession, debts, trust, and the
  like — stay as applications or extensions, unless
  shown to be forced by the basic questions or by
  implementing the core.
- Ideas that no longer hold up are removed, not left
  to accumulate, so the theory still reads as a single
  pass.
- Beauty is a check, not decoration: a reader should
  be able to look at the core and think, this is clear.
