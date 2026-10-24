# harness-research

Toward a unified, coherent theory of what a harness is.

## The question

What is a harness, at its core?

Not: what is the best harness this month, for this model, for this task.
But: what is the structure that all harnesses share, stated so clearly
that new developments fit into it without breaking it, and so simply
that the resulting code needs very little maintenance?

## The goal

Fit all of this under one simple, unified, mathematical
theory that can be easily implemented in code.

Its core should be so tight and generalizable that any
harness can build on top of it. That is the test for
what belongs in the core: if a claim cannot be stated
simply, defined mathematically, and implemented as code
that other harnesses can build on, it does not belong
in the core, however interesting it is elsewhere.

## The approach

Approach this as a mathematician, not as a developer or as an
AI researcher chasing the latest scaffold.

That means: look for definitions, invariants, and distinctions that
are true across examples, not for a list of tricks that happen to work
now. A theory should be internally consistent, grounded in examples,
and able to encompass developments that do not exist yet. If it cannot
do that, it is a description, not a theory.

It should be clear enough that a reader can tell at once what belongs in the core and what does not.

## How to read this repo

For the shape at a glance, open `visualization/index.html`.
It is an interactive diagram generated from the Rust stubs
in `harness-core`, so it changes when the stubs change.

Start in `docs/theory.md` for the theory itself. It states only the core: what a harness is, how to test a candidate, and how to adopt an existing harness. The high-level objectives and key results
are in `docs/OKR.md`. Removal counts as work: an idea that
no longer holds up should be taken out, not left to confuse.

## Repo structure

- `docs/theory.md` — the core theory, and its basic and open questions
- `docs/OKR.md` — long-term direction and monthly objectives
- `harness-core/` — rough Rust stubs matching the theory;
  no implementation, no tests
- `adaptations/` — stub adapters showing how open-source
  harnesses can be adopted on top of `harness-core`;
  no harness is run, no tests
- `visualization/` — interactive diagram of the harness
  shape, generated from `harness-core` by `generate.py`

The theory files live in `docs/`. Code lives in
`harness-core/` and `adaptations/`, and the diagram
lives in `visualization/`, each with its own README.

Care here is not a metric. It cannot be reduced to a count of notes,
commits, or refactors, and optimizing such a count would go against
the point. Care is a disposition: consider the future reader, consider
the future maintainer, do not tack things on, do not let scope spiral,
do not let code tangle. Every addition is a liability that future
work inherits, so an addition has to earn its keep.
