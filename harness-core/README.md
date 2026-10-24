# harness_core — stubs only

A rough Rust package whose shapes match `docs/theory.md`. It is
not an implementation, and it has no tests.

Each module corresponds to a part of the theory:

- `boundary` — the designer-control boundary
- `state` — the state split, `s = (w, h)`
- `action` — the action space at levels of abstraction
- `write_permission` — agent write permission over harness state, fixed core and modification policy
- `observation_functions` — one observation function per principal
- `precedence` — designer-fixed precedence over transitions, veto not compulsion
- `membership` — what counts as a harness, and what does not
- `run_loop` — the policy loop itself
- `adoption` — how an existing harness is adopted on top of this core; the first adapter is SWE-agent

Functions marked with `todo!()` are stubs: their signatures
are the claim, their bodies are deliberately empty. A trait
with no method bodies is likewise a stub interface, to be
filled by a future implementation or by an adapter for an
existing harness.

Nothing here runs a harness. That is intentional.

The interactive diagram in `../visualization/` is
generated from these stubs: run
`python3 visualization/generate.py` from the repo root
after changing a stub, and the diagram updates.
