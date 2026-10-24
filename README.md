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

It should also be beautiful to read. Someone should be able to look at
it and think, this is clear. Beauty here is not decoration. A clear
mental model is what makes code flexible later, because when a new
development arrives, you know exactly where it belongs, or whether it
belongs at all.

## How to read this repo

Start in `theory.md`. The log of what was tried, found, ruled out,
and left open is in `log.md`. Removal counts as work: an idea that
no longer holds up should be taken out, not left to confuse.

Care here is not a metric. It cannot be reduced to a count of notes,
commits, or refactors, and optimizing such a count would go against
the point. Care is a disposition: consider the future reader, consider
the future maintainer, do not tack things on, do not let scope spiral,
do not let code tangle. Every addition is a liability that future
work inherits, so an addition has to earn its keep.
