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

## 2026-10-24 — Membership

Membership test: harness runs a policy's inner loop, is designer-controlled, acts on w or h, has a designer-fixed control outside the policy's write set. Macedo T1-T4 corrected: harness-directed acts count; agent-rewritable caps are not control; weak context is bad, not absent. Open: goodness.

## 2026-10-24 — Basic questions

Basic questions added: action space, POMDP fit, major harnesses today, steps and ending, state vs context, tools vs subagents, harness vs framework, errors, costs and permissions, human intervention, and variation across harness kinds.

## 2026-10-24 — Action space as a tower

Action space as a tower, not a set: tokens, parsed moves, and effects, with A depending on harness state h. Tools are single-step moves, subagents moves running an inner loop; skills, prompts, memories parametrize A and observation, not moves. Open: cost of a move at each level.

## 2026-10-24 — Goal

Goal added to README: one simple, unified, mathematical theory, easily implemented in code, with a core so tight and generalizable that any harness can build on top of it.

## 2026-10-24 — OKR

High-level OKR added: one simple unified theory, grounded in harnesses used today, easily implemented and buildable on, and kept tight as it grows. Key results are checkable outcomes, not counts.

## 2026-10-24 — SWE-agent worked example

SWE-agent worked through the membership test in theory.md: loop, designer config, bash-plus-bundles action set, cost/timeout/blocklist controls, history rewriting, and submit kept apart from SWE-bench grading. Open: human intervention and live overseer views.

## 2026-10-24 — Rust stubs and monthly OKRs

Rust stubs and monthly OKRs: rough harness-core package in Rust with stubs matching the theory, no implementation or tests; OKR rescoped to Month 1: adopt one popular harness in stubs, align stubs and theory, clean the minimal theory.

## 2026-10-24 — Visualization and organization

Repo organized with a structure map in README; interactive visualization added in visualization/, generated from harness-core stubs by generate.py, so it updates when stubs change. No theory files moved.

## 2026-10-24 — Move theory files to docs

Theory files moved to docs/: theory.md, log.md, and OKR.md now live in docs/, with README and code references updated. Content unchanged.

## 2026-10-24 — SWE-agent stub adoption

SWE-agent adopted in stubs: SweAgent implements AdoptedHarness with loop, A(h), state split, views, controls, ending, a passing membership result, and gaps left open; theory's worked example points at it.

## 2026-10-24 — Adaptations folder

Parallel adaptations crate added with stub adapters for Claude Code, Codex, Cursor, Gemini CLI, Copilot, OpenCode, and SWE-agent, each mapping loop, action space, state, views, controls, ending, and membership onto harness-core, with gaps to verify.

## 2026-10-24 — Open-source-only adaptations

Removed closed-source adapters Claude Code, Cursor, and Copilot from adaptations/; narrowed Codex to its Apache 2.0 CLI only. Remaining adapters are all open source: Codex CLI, Gemini CLI, OpenCode, SWE-agent.

## 2026-10-24 — POMDP verdict

POMDP verdict: a single tuple is one step's shape, not a harness theory. A run fits a POMDP per level, epoch, and principal, under designer vetoes; tower, per-principal views, aspiration, and trace judgement fill what one tuple cannot hold.

## 2026-10-24 — Basic questions as consideration, not resolution

Basic questions reframed: they require explicit consideration, not a resolved answer. theory.md intro and OKR.md updated to match: a question counts as addressed when its bearing, assumptions, and open remainder are stated, even with no settling answer in sight.

## 2026-10-24 — Human intervention

Basic question 10 considered, not resolved: pause, steer/queue, edit, and policy replacement are four acts with different landing sites and timings, checked against OpenCode's session interrupt, inbox, and revert code. Open: precedence, settlement window, partial-step resumption.

## 2026-10-24 — Codex CLI escalation verified

Codex CLI escalation verified in outline against openai/codex at 970111f6: the agent's request_permissions call is decided by the guardian reviewer or, failing that, by a user event, and a grant is scoped to the turn or the session, never written to disk; under never mode it is refused without asking. Verdict: not arbitration in the theory's sense, but a judging party's veto with a scope choice. Kept apart: a user approval may also persist an exec-policy amendment on disk, changing the rule itself. Open: what the guardian decides on its own.

## 2026-10-24 — Codex CLI sandbox and approval verified

Codex CLI adapter verified in outline against openai/codex at 970111f6: sandbox modes and policy, exec-policy allow/prompt/forbidden, and approval modes untrusted/on-request/granular/never. Finding: under default on-request, approval-seeking is model-chosen, so sandbox enforcement and forbidden rules, not the asking, are the controls that hold regardless. Open: per-OS enforcement, approval records, escalation vs arbitration.

## 2026-10-24 — OpenCode interruption settlement and partial steps settled

OpenCode intervention, re-checked against anomalyco/opencode at 17b79f4e0c16dcff200b0417567ed22aeb2d7166 (the commit the theory already cited). Two opens from the human-intervention section are now settled, and one earlier claim corrected. Settlement: a session counts as active through interruption cleanup and terminal settlement; a second interrupt while stopping, or in the settlement window after the owner exited, returns false, and in the settlement window it still claims recorded wakes so no successor starts for a dead intent; wakes arriving during cleanup are new admissions and start a successor at settle; settlement, not acceptance, is obtained by composing interrupt with awaitIdle. Partial steps: resumption after a user interruption restarts the step, it does not continue it — the step fails as aborted, unsettled tools fail, completed tool results survive in history, and the next drain starts a new step. Correction: the "continue from where you left off" observation belongs to interrupted provider streams recovered inside a step, not to user interruption, where no such instruction is injected. Open: intervention precedence, and the batch case.

## 2026-10-24 — Error model

Basic question 8 considered, not resolved: failures sorted by whether a move was attempted, who refused or failed it (control, world, harness, policy), and whether the run continues, retries bounded, or ends, using SWE-agent, Codex CLI, and OpenCode accounts already in the repo. Also corrected the SWE-agent adapter path in theory.md to adaptations/src/swe_agent.rs. Open: retry bounds, timeout classification, refusal handling.

## 2026-10-24 — Ending versus termination

Resolved reviewer gap 1: "ending" now means harness retirement only. AdoptedHarness::ending renamed to termination, adapters and theory updated, vocabulary rule stated in the Ending section. Open: reviewer gap 2, arbitration silence in Gemini CLI and OpenCode adapters.

## 2026-10-24 — Arbitration statements in adapters

Resolved reviewer gap 2: the adapter specifications now agree that every adapter must state its arbitration position in controls() — the rule, that there is none, or that it is not yet stated — and every adapter does: SWE-agent and Codex CLI state none, Gemini CLI and OpenCode state not yet stated. "Not yet stated" counts as a statement, not silence, so a reader can tell absence from an unnamed rule. Open as adapter questions, not specification conflicts: whether Gemini CLI has an arbitration rule, and OpenCode's intervention precedence.

## 2026-10-24 — Cost, budgets, latency, permissions

Basic question 9 considered, not resolved: cost, budget, latency, and permission sorted into four places — cost recorded in trace/h, budget as a core threshold, latency as per-move and per-run clocks plus a liveness timer, permission as admission, veto, or approval shaping A(h) — using only SWE-agent and Codex CLI facts already checked in the repo. No new core claim. Open: whether the agent's view should carry running cost, subagent cost attribution, and temporary vs durable permission change.

## 2026-10-24 — One theory across harness kinds

Basic question 11 considered, not resolved: coding, browsing, chatting, and operating harnesses pass the same kind-blind membership test, but the w/h line, the principal count, termination versus verdict, and the adequacy of post-hoc review all change shape by kind. Chatting makes the interlocutor a principal, not world; browsing makes moves world-offered and page content untrusted world; operating and chatting make many moves irreversible, so control must be pre-emptive. No new core claim, and the section is prospective: all repo adapters are coding, so no particular non-coding product is claimed. Open: A(w, h) notation, principal-judged verdict vocabulary, and the non-coding second adoption that would test the section.

## 2026-10-24 — Ending vocabulary narrowed, gap 3 resolved

Resolved reviewer gap 3 by narrowing the rule rather than renaming every verb use: the collision was always the bare noun "ending", not verb phrases naming their object. The Ending section and the AdoptedHarness::termination doc now reserve the bare noun for harness retirement and state that "ends the run" / "ends the conversation" are not collisions, because the object disambiguates; "terminates" stays the preferred verb for a run. The one remaining bare-noun collision, the SWE-agent adapter's "Ending and verdict come apart" gaps entry, is renamed to "Termination and verdict come apart", mirroring the theory's worked-example heading. No cargo toolchain available in this run, so the stub crates were not compile-checked; the code change is a single string literal. Open: whether the reviewer accepts the narrowed rule, or wants the verb uses renamed after all.

## 2026-10-24 — OpenCode intervention precedence settled

Intervention precedence settled against anomalyco/opencode at the same pinned commit as the earlier intervention work (17b79f4e). Finding: there is no ranking of principals and no overseer interrupt — the coordinator's only interrupt reasons are user and shutdown, and the first interrupt wins while a second returns false, so user versus shutdown is decided by arrival order, not rank. The other pairs are decided per mechanism: interrupt over in-flight work, active execution over revert (busy error), steer over queue at promotion, permission deny over a move, and a user reply over the whole pending permission batch; a bare decline is a typed failure the model does not continue from, while a decline with feedback becomes a tool failure it continues from. The OpenCode adapter's arbitration statement and gaps now say this instead of "not yet stated". Open: the batch case, where intervention may collapse to starting a new run. Note: in this run ssh read known_hosts from /root (the process user) instead of /home/hatch, so GitHub SSH needed an explicit UserKnownHostsFile pointing at /home/hatch/.ssh/known_hosts; the server key matched the stored GitHub key.

## 2026-10-24 — Harness versus scaffold, workflow, application, agent

Basic question 7, continued in the Membership section: scaffold, workflow, application, and agent-as-product were the neighbours the section had not named — framework, SDK, plugin, orchestrator, and evaluation harness were already there. No new core claim, and no new neighbour type: a scaffold is a harness under another name when it runs the loop, material when it is starter code; a workflow is an orchestrator under another name, a loop only for the stretch an observation alters; an application and an agent-as-product are bundles, and the test applies to the harness part, not the bundle. A policy on its own is not a harness. Open, for the user: the standing instruction to cut theory.md to 40% — the prose inspected this run is dense argument, not wordy filler, so reaching 40% needs a structural decision about which blocks to move or summarise, not line trimming in a single tick.

## 2026-10-24 — Theory trimmed to about 40%

Theory trimmed to about 40% by removing repeated restatements, throat-clearing, and secondary elaborations. Headings, core claims, worked example, basic/open questions, and sources kept; the new Membership neighbours passage retained in short form.

## 2026-10-24 — Gemini CLI arbitration verified

Gemini CLI arbitration verified in outline against google-gemini/gemini-cli at cd1b45c2: no arbitration in the theory's sense. The policy engine decides one proposed move at a time (ALLOW/DENY/ASK_USER, first matching rule by priority, admin over user over workspace over extension over default); confirmation is raced between terminal and IDE companion, first response winning; hooks deny if any hook blocks. All three are veto/approval over one move, not a choice between contending proposals. The adapter's arbitration gap is closed; sandbox/approval, extension, and multi-model gaps remain.

## 2026-10-24 — Theory rewritten in ASD-STE100

Theory rewritten in ASD-STE100 Simplified Technical English: short sentences, active voice, simple tenses, one term per concept. Math symbols kept verbatim and defined before first use; headings, claims, questions, and sources kept.

## 2026-10-24 — Principal index renamed in STE theory

Audited the STE rewrite against its own rule that no symbol is introduced without a definition: p was defined in Starting definitions as the probability function, then reused in Multiple principals as the name of a principal (p_p, o_p). Renamed the principal index to i, defined by "Let i name one principal", so p stays the probability function throughout and the family reads p_i(o_i | s). No other symbol collision found in the audit; G is redefined across sections (fixed point, set of goals, goal) and is left as an open question for the reviewer rather than renamed in this tick.

## 2026-10-24 — G unified as the set of goals

Resolved the symbol overload left open by the principal-index tick: G meant three things — a fixed goal point (POMDP section), the set of goals for the run (Goals section), and the goal treated as an aspiration (epochs section). G is now defined once in Starting definitions as the set of goals for the run, before its first use. The POMDP passage now calls a single fixed goal point the old picture that aspiration and epochs replace, and the epochs section now treats each version of G as an aspiration, with G_i defined as the version of G in epoch i. No claim changed: the Goals section already used the set-of-goals sense, and the sources entry already described satisficing as an aspiration in place of a point.
