# harness_core — stubs only

A rough Rust package whose shapes match `theory.md`. It is
not an implementation, and it has no tests.

Each module corresponds to a part of the theory:

- `boundary` — the designer-control boundary
- `state` — the state split, `s = (w, h)`
- `action` — the action space as a tower of levels
- `authorship` — agent authorship over harness state
- `views` — one observation function per principal
- `arbitration` — designer-fixed arbitration over transitions
- `liveness` — liveness over histories, progress, epochs
- `meta` — the meta-rule: recorded, prospective, visible design acts
- `membership` — what counts as a harness, and what does not
- `run_loop` — the policy loop itself
- `adoption` — how an existing harness is adopted on top of this core
- `extensions` — applications of the core, not the core itself:
  succession, ending, debts, identity, trust, and roots

Functions marked with `todo!()` are stubs: their signatures
are the claim, their bodies are deliberately empty. A trait
with no method bodies is likewise a stub interface, to be
filled by a future implementation or by an adapter for an
existing harness.

Nothing here runs a harness. That is intentional.
