#!/usr/bin/env python3
"""Generate visualization/index.html from the Rust stubs.

The visualization is not hand-drawn data. It is derived,
every time this script runs, from harness-core/src/*.rs:

- each module becomes a node,
- its //! module documentation becomes the node's description,
- its pub structs, enums, traits, and functions become the
  node's stub list,
- the mapping from module to theory summary comes from
  harness-core/README.md.

So when a stub changes, re-run this script from the repo
root and the diagram updates:

    python3 visualization/generate.py

Do not edit visualization/index.html by hand. Edit the
stubs (or harness-core/README.md) and regenerate.
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "harness-core" / "src"
PKG_README = ROOT / "harness-core" / "README.md"
OUT = Path(__file__).resolve().parent / "index.html"

# Diagram layout: (x, y, layer)
# The middle row is the loop. The top and bottom rows are
# the things that govern, shape, or judge that loop.
LAYOUT = {
    "boundary": (40, 50, "governs"),
    "precedence": (410, 50, "governs"),
    "write_permission": (730, 50, "governs"),
    "run_loop": (40, 270, "loop"),
    "action": (280, 270, "loop"),
    "state": (520, 270, "loop"),
    "observation_functions": (760, 270, "loop"),
    "membership": (280, 490, "governs"),
    "adoption": (520, 490, "bridge"),
}

TITLES = {
    "boundary": "Boundary",
    "precedence": "Precedence",
    "write_permission": "Write permission",
    "run_loop": "Policy loop",
    "action": "Action levels",
    "state": "State s = (w, h)",
    "observation_functions": "Observation functions",
    "membership": "Membership",
    "adoption": "Adoption",
}

# Solid loop edges, and dashed governing edges.
EDGES = [
    ("run_loop", "action", "solid", "policy chooses"),
    ("action", "state", "solid", "transition"),
    ("state", "observation_functions", "solid", "state rendered"),
    ("observation_functions", "run_loop", "solid", "observation o"),
    ("boundary", "state", "dashed", "who controls what"),
    ("write_permission", "state", "dashed", "agent writes h"),
    ("precedence", "action", "dashed", "veto, not compulsion"),
    ("membership", "run_loop", "dashed", "is this a harness?"),
    ("adoption", "run_loop", "dashed", "existing harness, mapped"),
]


def module_doc(path: Path) -> str:
    lines = []
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if stripped.startswith("//!"):
            lines.append(stripped[3:].strip())
        elif lines:
            break
    return " ".join(lines).strip()


def stub_items(path: Path) -> list[str]:
    text = path.read_text()
    items: list[str] = []
    for kind, name in re.findall(
        r"pub\s+(struct|enum|trait|fn)\s+([A-Za-z_][A-Za-z0-9_]*)", text
    ):
        label = f"{kind} {name}"
        if label not in items:
            items.append(label)
    return items


def pkg_mapping() -> dict[str, str]:
    mapping: dict[str, str] = {}
    for line in PKG_README.read_text().splitlines():
        m = re.match(r"- `(\w+)` — (.+)$", line.strip())
        if m:
            mapping[m.group(1)] = m.group(2).strip()
    return mapping


def main() -> int:
    mapping = pkg_mapping()
    modules = []
    for name, (x, y, layer) in LAYOUT.items():
        path = SRC / f"{name}.rs"
        if not path.exists():
            print(f"warning: stub missing, skipping: {path}", file=sys.stderr)
            continue
        modules.append(
            {
                "id": name,
                "title": TITLES[name],
                "layer": layer,
                "x": x,
                "y": y,
                "file": f"harness-core/src/{name}.rs",
                "theory": mapping.get(name, ""),
                "description": module_doc(path),
                "stubs": stub_items(path),
            }
        )
    data = {"modules": modules, "edges": EDGES}
    html = TEMPLATE.replace("__DATA_JSON__", json.dumps(data, indent=2))
    OUT.write_text(html)
    print(f"wrote {OUT} ({len(modules)} modules from {SRC})")
    return 0


TEMPLATE = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Harness — shape at a glance</title>
<style>
  body { font-family: system-ui, -apple-system, sans-serif; margin: 24px auto; max-width: 1040px; padding: 0 16px; color: #1a1a1a; line-height: 1.45; }
  h1 { font-size: 1.6rem; margin-bottom: 0.2rem; }
  p.lede { color: #555; max-width: 760px; }
  .filters { margin: 16px 0; display: flex; gap: 8px; flex-wrap: wrap; }
  .filters button { cursor: pointer; border: 1px solid #888; background: #fff; border-radius: 20px; padding: 6px 14px; font-size: 0.95rem; }
  .filters button.active { background: #1a1a1a; color: #fff; }
  svg { width: 100%; height: auto; border: 1px solid #ddd; border-radius: 12px; background: #fafafa; }
  .node { cursor: pointer; }
  .node rect { fill: #fff; stroke: #333; stroke-width: 1.5; rx: 12; }
  .node.loop rect { fill: #eef4ff; }
  .node.governs rect { fill: #fff8e6; }
  .node.bridge rect { fill: #eefbf0; }
  .node.extension rect { fill: #f3eefb; }
  .node.selected rect { stroke-width: 3.5; stroke: #0057d9; }
  .node.dim { opacity: 0.25; }
  .node text { font-size: 15px; font-weight: 600; text-anchor: middle; }
  .node .sub { font-size: 12px; font-weight: 400; fill: #555; }
  .edge-label { font-size: 11px; fill: #666; text-anchor: middle; }
  #detail { border: 1px solid #ddd; border-radius: 12px; padding: 16px 20px; margin-top: 16px; }
  #detail h2 { margin-top: 0; }
  #detail code { background: #f0f0f0; padding: 1px 5px; border-radius: 4px; }
  #detail ul { columns: 2; }
  .note { color: #666; font-size: 0.9rem; }
</style>
</head>
<body>
<h1>The shape of a harness, at a glance</h1>
<p class="lede">The middle row is the loop: a policy chooses, an action executes,
state changes, an observation function renders an observation, and the loop runs again.
The rows around it govern, shape, or judge that loop. Click any node
to see what it is, what theory it comes from, and which Rust stubs
in <code>harness-core</code> it corresponds to.</p>
<div class="filters" role="group" aria-label="Filter nodes">
  <button data-filter="all" class="active">All</button>
  <button data-filter="loop">Loop only</button>
  <button data-filter="governs">Governing only</button>
  <button data-filter="core">Core (no extensions)</button>
</div>
<svg id="diagram" viewBox="0 0 980 600" role="img" aria-label="Harness diagram"></svg>
<div id="detail"><p>Click a node above to see its description and stubs.</p></div>
<p class="note">Generated from <code>harness-core/src/*.rs</code> by
<code>visualization/generate.py</code>. Do not edit this file by hand:
change a stub, re-run the generator, and this diagram updates.</p>
<script>
const DATA = __DATA_JSON__;
const svg = document.getElementById('diagram');
const detail = document.getElementById('detail');
const NS = 'http://www.w3.org/2000/svg';
const W = 180, H = 80;
const byId = Object.fromEntries(DATA.modules.map(m => [m.id, m]));
function el(tag, attrs, parent) {
  const e = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, v);
  parent.appendChild(e);
  return e;
}
function centre(m) { return [m.x + W / 2, m.y + H / 2]; }
// Edges first, so nodes sit on top.
for (const [a, b, kind, label] of DATA.edges) {
  const A = byId[a], B = byId[b];
  if (!A || !B) continue;
  const [x1, y1] = centre(A), [x2, y2] = centre(B);
  let d;
  if (a === 'observation_functions' && b === 'run_loop') {
    d = `M ${x1} ${y1 + H / 2} C ${x1} ${y1 + 180}, ${x2} ${y2 + 180}, ${x2} ${y2 + H / 2}`;
  } else {
    d = `M ${x1} ${y1} L ${x2} ${y2}`;
  }
  const p = el('path', { d, fill: 'none', stroke: kind === 'solid' ? '#333' : '#999',
    'stroke-width': kind === 'solid' ? 2 : 1.5, 'stroke-dasharray': kind === 'solid' ? '' : '6 5',
    markerEnd: 'url(#arrow)' }, svg);
  p.dataset.from = a; p.dataset.to = b; p.classList.add('edge');
  const mx = (x1 + x2) / 2, my = (y1 + y2) / 2 + (a === 'views' ? 90 : -8);
  const t = el('text', { x: mx, y: my, class: 'edge-label' }, svg);
  t.textContent = label; t.dataset.from = a; t.dataset.to = b; t.classList.add('edge');
}
const defs = el('defs', {}, svg);
const marker = el('marker', { id: 'arrow', markerWidth: 8, markerHeight: 8, refX: 7, refY: 3, orient: 'auto' }, defs);
el('path', { d: 'M0,0 L7,3 L0,6 Z', fill: '#333' }, marker);
for (const m of DATA.modules) {
  const g = el('g', { class: `node ${m.layer}`, transform: `translate(${m.x},${m.y})`, tabindex: 0 }, svg);
  g.dataset.id = m.id;
  el('rect', { width: W, height: H, rx: 12 }, g);
  const t1 = el('text', { x: W / 2, y: 34 }, g); t1.textContent = m.title;
  const t2 = el('text', { x: W / 2, y: 56, class: 'sub' }, g); t2.textContent = m.id + '.rs';
  const show = () => {
    document.querySelectorAll('.node').forEach(n => n.classList.remove('selected'));
    g.classList.add('selected');
    const stubs = m.stubs.length ? `<ul>${m.stubs.map(s => `<li><code>${s}</code></li>`).join('')}</ul>` : '<p>No public stubs yet.</p>';
    detail.innerHTML = `<h2>${m.title}</h2>
      <p><strong>Stubs:</strong> <code>${m.file}</code></p>
      <p>${m.description || ''}</p>
      <p><strong>Theory:</strong> ${m.theory || ''}</p>
      <p><strong>Stub items in this module:</strong></p>${stubs}`;
  };
  g.addEventListener('click', show);
  g.addEventListener('keydown', e => { if (e.key === 'Enter' || e.key === ' ') show(); });
}
document.querySelectorAll('.filters button').forEach(btn => {
  btn.addEventListener('click', () => {
    document.querySelectorAll('.filters button').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    const f = btn.dataset.filter;
    document.querySelectorAll('.node').forEach(n => {
      const m = byId[n.dataset.id];
      let keep = true;
      if (f === 'loop') keep = m.layer === 'loop';
      if (f === 'governs') keep = m.layer === 'governs';
      if (f === 'core') keep = m.layer !== 'extension' && m.layer !== 'bridge';
      n.classList.toggle('dim', !keep);
    });
    document.querySelectorAll('.edge').forEach(e => {
      const A = byId[e.dataset.from], B = byId[e.dataset.to];
      let keep = true;
      if (f === 'loop') keep = A.layer === 'loop' && B.layer === 'loop';
      if (f === 'governs') keep = A.layer === 'governs' || B.layer === 'governs';
      if (f === 'core') keep = A.layer !== 'extension' && A.layer !== 'bridge' && B.layer !== 'extension' && B.layer !== 'bridge';
      e.style.opacity = keep ? 1 : 0.15;
    });
  });
});
</script>
</body>
</html>
"""

if __name__ == "__main__":
    raise SystemExit(main())
