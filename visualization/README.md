# Visualization

`index.html` is an interactive diagram of the shape of a
harness: the policy loop in the middle, and the things
that govern, shape, or judge that loop around it.

It is generated, not hand-drawn. `generate.py` reads
`harness-core/src/*.rs` and `harness-core/README.md`
every time it runs, and builds the diagram from them:
each module becomes a node, and its public structs,
enums, traits, and functions become the node's stub list.

To update it after changing a stub, run from the repo
root:

    python3 visualization/generate.py

Then open `visualization/index.html` in a browser. It
needs no server and no network: all data is embedded
in the file at generation time.

Do not edit `index.html` by hand. Edit the stubs (or
`harness-core/README.md`) and regenerate, so the
diagram cannot drift from the code it describes.
