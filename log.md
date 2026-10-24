# Log

A record of what was tried, found, ruled out, and left open.

## 2026-10-24 — Seed

POMDP framing: harness is environment, carrying out transitions p(s'|s,a) and observations p(o|s), judged against goals G. First example: RLM as agent control over observation. Approach: mathematician's, not scaffold-chasing. Open questions listed in theory.md.

## 2026-10-24 — Boundary by designer control

Sutton and Barto ch. 3, turned further: split environment by who can change what — agent (policy), harness (designer-controlled), world (neither). System prompt is harness. Ruled out: harness = everything outside the agent. Open: self-rewriting harness.

## 2026-10-24 — Self-modifying harnesses

Prime Agent's Continual Harness: fixed core plus mutable supplement (notes, skills). Designer controls core and policy. Attribution survives iff modification machinery is outside the agent's write set. Open: learned policy.

## 2026-10-24 — World- and harness-directed actions

State s=(w,h). Actions sort by effect: world-directed (change w) vs harness-directed (change h). Observation and transition control are one principle: agent authorship over h. Legibility to other principals stays separate.

## 2026-10-24 — Multiple principals, one state

One state s, observations p_p(o_p|s) per principal: agent, user, overseer. Views must satisfy sufficiency and separation (seeing is not writing). Agent writes to h change later views, so modification policy is also visibility policy. Open: conflict.

## 2026-10-24 — Arbitration is harness

Arbitration is harness, not a principal: a designer-fixed core rule on transitions, a partial order on action pairs. Veto, not compulsion. Preemption must be legible to the preempted principal. Uncovered conflicts suspend that transition. Open: liveness.

## 2026-10-24 — Liveness is history

Safety is step-local, liveness is history (Alpern-Schneider 1985). Failures: deadlock, starvation, livelock. State is unbounded, so judge traces: counters/timers with designer thresholds flip 'held' to 'defective'. No new principal needed. Open: progress.

## 2026-10-24 — Vague or shifting G

Progress is approach to current aspiration (Simon 1955), not distance to ideal G. Proxies count only if tied to aspiration. Revisions start recorded epochs; churn without completion is cross-epoch liveness failure. Open: threshold-setter if designer is absent.

## 2026-10-24 — Restoration after deletion

Three commits replaced README, theory, and log with deletion notices, citing rule violations. Restored here in compliant form: theory reads as a single pass, each log entry under 300 characters. Full prior text remains in commit history.

## 2026-10-24 — Absent-designer thresholds

Absent-designer thresholds: inherited values stay binding. Agent-set or overseer-as-judge changes are evasion; learned ones confuse norm with distribution. A core meta-rule computes values, never from the defect judged. Freeze if uncovered; revise as a recorded design act. Open: succession.

## 2026-10-24 — Succession

Succession is legitimate only from a core rule fixed before absence: a named successor or a fixed procedure, on a legible trigger. Self-appointment is evasion. No successor means freeze, then sunset, not improvisation. Open: when ending is right.

## 2026-10-24 — Ending

Ending is a design act, not a verdict: completion, supersession with handover, or sunset. Only the designer role may end a harness. An ending is announced, drained, and archived; records just stopping is abandonment, not ending. Open: what a successor harness owes the ended one's principals.

## 2026-10-24 — Successor's debts

Supersession inherits outstanding obligations, not just records: in-flight epochs, standing commitments, no new burden without notice. Decline is recorded, not punished. Open: debts passing down a chain of successors.

## 2026-10-24 — Chained succession

Chained succession: debts are owed by the office, not the holder, so they pass intact. Transfer renews no term, drops no debt silently, launders no title. Handovers without debt resolution are churn. Open: how much revision before a successor is a different harness?

## 2026-10-24 — Identity across revision

Identity across revision: a harness survives total code change if authority, revision, and debt traces stay intact; changing recognition rules by ordinary act founds a new harness. Open: trust in the records themselves.

## 2026-10-24 — Trust in the records

Trust in the records: provenance, hash-chained tamper-evidence, independent views, and anchoring outside the harness give a record its force. Competing ledgers are settled by continuity; an unruled fork suspends the contested claims. Open: where the anchor regress stops.

## 2026-10-24 — Where the anchor regress stops

Anchor regress: anchors claim only existence and order, not truth. Regress ends at the first store no disputant controls, or at a root a principal holds directly — key, copy, witness. Cycles ground nothing. Open: rotating a root without breaking chain continuity.

## 2026-10-24 — Rotating a root

Root rotation: old root vouches for the new while trusted, both live through an overlap, old retires by signed revocation. A compromised root cannot vouch; emergency rotation needs a reserve root or re-founding. Open: does the root ledger need its own anchor?

## 2026-10-24 — Root ledger needs no higher anchor

Root ledger needs no anchor beyond its roots: each link is vouched by the root live at its time, checkable forward from any held root; the segment before a held root rests on adoption, not proof; rewrites are caught by independent holders comparing copies. Open: return to the minimal core.

## 2026-10-24 — Minimal core

Seven load-bearing claims distilled: boundary, state split, authorship, views, arbitration, liveness, meta-rule. Succession, ending, debts, identity, trust, anchors are applications, not axioms. Open: what breaks if one core claim is dropped?

## 2026-10-24 — Restoration after deletion

The 09:28 tick deleted theory.md with no stated reason; restored here verbatim from the prior commit, since README directs readers to it. The deletion itself remains visible only in commit history.

## 2026-10-24 — What each claim carries

Drop-tested the minimal core: removing any one of the seven claims loses something the other six cannot recover: failure's address, action's landing site, self-shaping, a principal's view, non-principal decision, judgement over time, a rule about rules.
