# Theory

This states the starting definitions, and the questions they leave open.

## Starting definitions

An agent is a policy `pi(a | o)`, with `o` the observation — in the LLM case, the full context history. An action `a` is, at the lowest level, a token or log-probability the network generates. The harness is part of the environment. It carries out the transition `p(s' | s, a)` and the observation `p(o | s)`. A tool is an engineered shortcut: the harness jumps to a state containing the answer, and shows it to the agent. Today's weather is the dummy case.

## The harness boundary: what the designer controls

Sutton and Barto draw the boundary by control: what the agent cannot change arbitrarily is environment. The same rule draws the harness boundary, splitting the environment three ways by who can change what. The agent is what the agent controls, its policy — for an LLM, its weights, not its prompt. The harness is what the designer controls without changing the agent or the world: the transition and observation machinery, internal state, interfaces, budgets, and checks. The world is what neither controls: weather, external service answers, retrieved documents. The test: can the designer change it arbitrarily, with agent and world fixed? Retraining the agent makes it agent-side; changing the world makes it world-side; otherwise it is harness. This rules out "harness = everything outside the agent": it collapses the designer's lever into the world, making failure unattributable. The system prompt is harness, not agent: the designer can rewrite it without touching the weights.

## The boundary, one level up: self-modifying harnesses

A self-rewriting harness — Prime Agent's Continual Harness is the concrete case — looks like a counterexample, since designer and agent cease to be separate parties. It is not one, applied at the right level: split the harness into a fixed core and a mutable supplement. The core is the immutable base prompt, the runtime applying edits at turn boundaries, and versioning recording provenance and enabling rollback. The supplement is prompt notes, memories, skills, and subagent specifications. The agent's edits are ordinary actions, within a designer-fixed modification policy: what may be edited, when, on what evidence, with what provenance. So the test survives, one level up: the designer controls the core and the policy, not the running harness. Attribution survives too: a bad skill the agent wrote is agent-side; a bad core, a permissive policy, or a failed rollback is designer-side. Attribution holds exactly when modification machinery is outside the agent's write set: Prime Agent keeps its runtime outside the writable state. A harness learning its own modification policy from experience is not yet covered.

## The action space: a tower, not a set

No single set `A` serves every level of description. The tower gives each level its own action set, with the same POMDP shape, related by expansion: an executed action unfolds into a run at the level below. At the lowest level, the action set is the model's vocabulary `V`, one token per step. The policy `pi(a | o)` is a distribution over `V`; nothing else is an action here. At the next level, the harness groups token sequences into parsed moves `A1`: emit text, call a tool, spawn a subagent, or stop. `A1` is a quotient, not a subset, of token sequences: two spellings of one tool call are one action. The same string can be different moves under different parsers or registries. A move's identity is its parsed intent plus its landing site in `s`. A move executes into `w`, `h`, or both. A tool move takes one harness step; a subagent move runs a whole inner loop, returning only a summary. The inner loop is itself a harness. The action set is state-dependent: write `A(h)`, not `A`. Registry, permissions, skills, and budget — all harness state — add or remove moves. Installing a skill gains moves; losing a tool loses them; exhausting budget leaves no costly move. `A(h)` lists attempts, not guarantees: success can still depend on `w`. Tools and subagents are moves, differing only in execution: one step, or one loop. Skills, prompts, and memories are not moves, but harness state parametrising the tower. A skill changes moves and parsing; a prompt shapes the next observation; a note changes a later one. Installing a skill changes `A(h)` later. A retrieval result is an observation, not an action. A chat reply lands in the transcript in `h`, for another principal's view; landing site and audience differ. Unparseable token sequences are not failed actions, but no action at all. The harness must still respond — error, retry, or halt — by a transition rule in its error model, not its action space. Inverted, the harness code is the policy and the model is environment: the same tower, a different cut.

## Is a POMDP the right framework?

For one step, yes; for a harness, no: a POMDP is the shape of a step, not the theory of the steps. The agent never sees `w` directly; much of what it sees is designed `h`. But a POMDP is one tuple: one state, action, and observation set, one transition, one observation function, one reward. First, not one action set. The tower is a family of POMDPs, one per level, related by parsing and expansion. A rejected token sequence is in neither action set. A subagent move is a temporally extended action — an option in a semi-Markov process, not a one-step move. Second, not one observation function: each principal needs its own `p(o | s)` over the same `s`. Third, its functions are fixed; a harness's are not, except within an epoch. Registry, parser, and thresholds can change by later design acts. A blocklist is a veto, not a price. Aspiration and epochs replace a fixed point `G` with a moving target and its revision record. Within an epoch a run is a POMDP; across epochs it is a family indexed by design acts. Fourth, its observations are evidence; a harness's are designed artefacts. SWE-agent's history processor replaces older observations with an omission note, and the next observation is built from the rewritten trajectory. It fits only because trajectory lists are in `h`. A belief from a designed record is a belief about what the designer chose to show, not about `w`. Fifth, its judgement is a step reward; a harness's is partly over traces. Liveness — deadlock, starvation, livelock, progress against aspiration — is a whole-history property no step reward encodes without smuggling in the trace judgement. Finally, the vacuity: if `s` and `o` may each be the whole history, any process is Markov, and "a run is a POMDP" is true and empty. It is informative only for a fixed `s`: which POMDP, level, epoch, principal, vetoes? Keep the POMDP as local shape, one level, one epoch; the rest goes around it. A formalism that cannot say who may change its own transition function cannot be a theory of a designed thing.

## Goals

Fix an agent, and vary the harness to make reaching a set of goals
`G` easier. But `G` is hard to specify widely enough, especially where
people are involved. Some goals are not about agent performance at
all, for example: a human can easily monitor the health of the system.
Because `G` shifts, harnesses should generally be easy to modify.
A theory that cannot survive a change in `G` is not yet a theory.

## An example that clarifies the framing

In a Recursive Language Model (RLM), the long prompt lives as a
variable in a Python REPL. The root model sees only metadata about
it, writes code to inspect and decompose it, and recursively calls
sub-models on pieces. The power is not Python as such: the agent
gains control over its own next observation. Observation becomes endogenous and adaptive:
the agent explores, acts on what it learns, then explores again. An agent
that chooses what to see can cherry-pick, miss disconfirming
evidence, or get stuck in a plausible branch. Control over observation
therefore moves harness design up a level: what observation actions
are available, what do they cost, what forces a check?

## One principle or several? World-directed and harness-directed actions

Write
the state as a pair, `s = (w, h)`: world state `w`, what neither
designer nor agent controls, and harness state `h`, what the designer
controls. A world-directed action changes `w`, as in sending a message
or placing an order. A harness-directed action changes `h`, as in
writing a REPL variable, a memory, a skill, or a plan.

Control over observation is the harness-directed case seen from the
observation side: changing `h` changes what `p(o | s)` will deliver
next. Control over transitions is the same case seen from the
transition side: changing `h` changes which transitions are available
next. So the principle is: a harness gives the agent
partial authorship over harness state, and thereby over its own future
observations and transitions. In RLM, writing a REPL variable shapes
the next observation. The harness's job then shifts up a level, from
delivering observations to governing authorship: which harness state
the agent may write, at what cost, with what checks, and with what
provenance. The split is relative to the boundary drawn, not absolute.
Legibility to other principals is genuinely separate: it
is about views on `s`, not about the agent's actions on it.

## Multiple principals, one state

A principal is a
party with its own observation function, its own write set, and its own
goal. The agent observes the context history, writes harness state within
the permitted scope, and pursues `G`. The user observes an answer or
summary, writes instructions that set or revise `G`, and judges whether
the answer serves them. The overseer observes the trace, costs, checks,
and health of the system, writes the fixed core and the modification
policy (or halts and rolls back), and pursues a goal that is not the
task at all.

What multiplies is the observation: where a single-agent account wrote a single
`p(o | s)`, the multi-principal view writes a family, one function per
principal, `p_p(o_p | s)` for each `p`. In the weather case, one tool
call in `s` yields three observations: the agent gets the structured
result it can act on, the user gets "sunny, 21°", and the overseer gets
latency, cost, and whether the call succeeded.

A view must satisfy two conditions that pull in different directions.
Sufficiency: the view must give its principal enough to do its job.
Separation: seeing something must not confer writing it. When the agent
writes `h`, it changes what every principal will subsequently see, so
the designer must decide in advance which agent writes are visible to
whom. The modification policy is therefore also a visibility policy,
not only a write policy.

## Conflict between principals: arbitration is harness, not a principal

When principals' goals disagree, the harness decides, by a rule the
designer fixed in advance. Arbitration is harness, not another principal.
An arbitrator with a goal of its own would be one more principal.
What makes a resolution binding is a rule, fixed in the core, that
decides which transition actually occurs.

At a step, principals propose actions, and the core's arbitration
rule selects which proposed transitions are enabled; the rest are disabled
for that step. Precedence is then a partial order on action pairs, not a
total order on principals: the user can revise `G`, which outranks the
agent's continuation of the old goal, but cannot override a fixed-core
check, which outranks the user's urgency. Arbitration, in this form, is
veto, not compulsion. Ramadge and Wonham's supervisor disables a subset
of controllable events, but has no means of forcing the plant to generate
an event. Preemption must also be legible: the preempted principal's view
must show the preemption and the rule behind it. If two proposed actions
conflict and no fixed
precedence decides between them, the harness must not invent a precedence
at runtime. It suspends only the contested transition, lets unrelated work
proceed, and surfaces the conflict to the designer, acting through the
overseer.

## Liveness under precedence: safety is step-local, liveness is history

An overseer that always halts, or a check that always gates, starves the
user's goal forever without violating any precedence rule. Alpern and
Schneider's decomposition says
every trace property is the conjunction of a safety property and a
liveness property ("Defining Liveness", 1985). A safety property says
nothing bad happens, and a violation is already visible in some finite
prefix. A liveness property says something good eventually happens, and
every finite prefix can still be extended to satisfy it.

At the history level, deadlock is when
no transition is enabled, so nothing proceeds at all. Starvation is when
one principal's transitions are always disabled while others proceed, so
the run looks alive but one party never gets served. Livelock is when
transitions keep firing but `G` is never approached. The classical
precedent is Ramadge and Wonham's nonblocking supervisor, under which
every reachable state can reach a marked state. Liveness is judged over
traces, at runtime, not over the state graph, in advance. Runtime
detection is therefore a core rule
over histories, using counters
(how many consecutive times this transition has been preempted) and timers
(how long since the run last made progress toward `G`). When a counter or
timer crosses a threshold, the run's status flips from "held" to
"defective".

The designer fixes the criterion in the core: what counts as progress,
and what threshold marks a defect. The core computes the verdict from
the trace. The overseer acts on it — abort, escalate to the designer,
or relax the check — and those actions are already in the overseer's
write set. When the overseer is itself the preempting party, the verdict
must still be visible in the user's view. A liveness verdict is not a
transition proposal, so it does not enter arbitration; it is a status
change on the run.

## Progress toward a vague or shifting G: epochs, aspiration, and proxies

`G` is often vague, and it shifts. It is closer to Herbert Simon's
satisficing: a
principal holds an aspiration level — good enough, on criteria the principal
can judge — and search stops at the first state that clears it, not at an
optimum (Simon, 1955). Progress is therefore not distance to a point, but
approach to the current aspiration, judged in the principal's view.

Where even aspiration is hard to state, a proxy may be all the criterion
has: new information gained, or uncertainty reduced about the world or
about what `G` itself is. A proxy is legitimate only with a tie back to
the aspiration: the gain must be usable by a principal to judge `G`, or to
advance toward it. Write the goal
as a sequence, `G_0, G_1, ...`, each revision made by a principal with
the write set to revise it — typically the user. Each revision starts a
new epoch. Counters and timers run within an epoch, and reset only at a
recorded epoch boundary, with provenance.

Masking: a revision that redefines `G` to a state already reached does
not count as progress within the old epoch. If the revising principal
accepts the current state under the new goal, the run completes under
the new epoch, recorded as a revision, not as progress. A silent
redefinition that merely resets the timers without ending the run is
an evasion, not an epoch. Churn: the goal is revised again
and again, each revision resetting the timers, so no epoch ever goes
defective and no epoch ever completes. Count revisions, or time without a
completed epoch, and when a designer-fixed threshold is crossed the run
flips to defective. The progress predicate is part of the core.

## Thresholds without a designer: meta-rules, frozen values, and succession

The liveness and progress sections both assume something they did not
defend: a threshold, fixed by the designer, at which a counter or timer
flips a run from "held" to "defective". The thresholds already in the
core persist as
inheritance, and stay binding until they are legitimately revised. The
agent must not set its own thresholds. The overseer must not set
thresholds at runtime in its capacity as judge. The harness must not
learn thresholds from its own traces: a threshold is normative — it
says what should count as a defect, not what usually happens.

The designer fixes, in the core, a meta-rule: how a threshold value is
computed, from what inputs, within what bounds. The meta-rule carries
one sharp condition: the quantity being judged must not set its own
bar. It may not scale with the trouble itself.
Where no meta-rule covers the case, the default is freeze, not relax.
The last designer-set values stay in force. Revision is still possible,
but only as a design act. A threshold change must be recorded as a
designer act, with provenance, visible to every principal. It is
prospective only: it applies
to future traces, and does not erase the prior verdict or retroactively
clear the history that produced it. Absence itself must also be legible:
the views must show that thresholds are inherited and frozen, and that
the designer is not currently reachable. Succession — who legitimately
inherits the designer role itself — is treated in the next section.

## Succession: who inherits the designer role, and on what authority

The designer role is the authority to revise the core: the precedence rules, the progress predicate, the thresholds and their meta-rules, the modification policy, the views. Succession is the transfer of that meta-authority, not a transition proposal, not a view, and not a verdict. So it cannot be decided by arbitration, which gates proposals, or by a liveness verdict, which judges traces.
The succession rule must already be in the core before the absence it governs. The first legitimate form is a named successor: the core names a person, a role, or an ordered list, and authority passes down the list when the trigger is met. The second is a fixed procedure: the core names no person, but fixes how a successor is chosen — who initiates, who meets, within what time, by what vote or confirmation — and the successor chosen by that procedure inherits the role.

Self-appointment is evasion: a party that declares itself designer has no prior rule behind it, only its own say-so. The agent cannot succeed the designer, for the boundary reason: that would collapse the agent-designer separation the whole theory rests on, and put the judged party in charge of the bar. The overseer cannot promote itself from judge to designer in order to escape a verdict, for the same reason it cannot set its own thresholds: the judge would be judging its own promotion. A user majority at runtime cannot confer the role either, unless a core procedure says a vote confers it — and then it is the procedure, not the majority, that confers it.
A legitimate successor inherits the office, not just its powers. That means three inheritances together: the core as it stands, with its meta-rules and frozen values; the records, provenance and prior verdicts, which the successor cannot erase or retroactively clear; and the constraints on design acts themselves — recorded, prospective, and visible to every principal. A delegate acts within a mandate the designer still holds, and the designer can revoke it. A successor replaces the designer, and the old designer's revocation is no longer available.

Succession also needs a trigger fixed in the core, or it is usurpation with a name attached. Once it is met, the transfer must be legible, by the sufficiency condition: the views show that the designer is absent, that the trigger was met, and that this party now holds the role, under the prior rule.

If there is no successor, the frozen inheritance of the thresholds section becomes terminal: runs complete or suspend under the frozen core, and the vacancy is surfaced, not filled by whoever acts first.

## Ending: when a harness may properly end, and how it ends well

Ending a harness is not the same as halting a run. Ending the harness is retiring the standing machinery itself: no new runs, existing runs brought to a close, responsibilities handed on or discharged. The two senses are kept apart by vocabulary: the bare noun "ending" is reserved for retiring the harness, and a run terminates.

Ending is a design act, not a verdict. Ending is decided by the designer role, and it carries the constraints of every design act in this theory: recorded, prospective, and visible to every principal. A harness that simply stops, with no record of who decided or why, has not ended. It has been abandoned.

Only the designer, or a legitimate successor under the succession rule, may end the harness.

There are three legitimate endings, and they are different shapes, not three names for stopping.

The first is completion of purpose. The harness was built for a goal, the goal's epochs complete, and no new epoch is opened.

The second is supersession. A new harness takes over the purpose, and the old one ends by handing over, not by vanishing. What passes is the same three inheritances as succession: the core as it stands, the records and prior verdicts, and the constraints on design acts.

The third is sunset. The designer or successor judges that the purpose itself is no longer worth the cost of standing ready for it. It must be made as a design act, recorded with its reasons, like any threshold revision.

An ending is announced, with a date, before it takes effect, so principals can finish their epochs or move. The harness drains rather than cutting off: it stops accepting new runs, lets in-flight runs complete or checkpoint at a clean boundary, and only then stops. And the records are archived, not erased.

## What a successor owes: debts, not just records

When a harness ends by supersession, does the successor inherit only the records, or also the obligations — the in-flight epochs, the standing commitments, the promises the old harness made and has not yet kept? A successor that takes the core, the records, and the constraints, but disowns what the old harness owed, has used supersession to default.

So the three inheritances of succession gain a fourth in supersession: outstanding obligations. A debt transfers only if it is in the record: an epoch in flight, a commitment made as a design act with a stated term, a checkpoint or handover promised to a principal.

Three debts, in the order a successor meets them.

First, in-flight epochs. An epoch that began under the old harness and has not completed does not restart silently under the successor, and it is not dropped. The successor either completes it under the terms the old harness set — the same aspiration, the same progress predicate, unless a design act revises them visibly — or brings it to a clean checkpoint and announces the transfer, so the principal's epoch continues across the handover rather than beginning again.

Second, standing commitments. A commitment the old harness recorded — a scheduled run, a promised observation, a handover to a named party, a standing cost or access arrangement — is honoured until its recorded term, or renegotiated.
Third, no new burden without notice. Transfer may not silently add to what a principal owes: a new cost, a new visibility, a new condition on an epoch already in flight. A successor may set new terms for new epochs — that is its design authority — but terms for epochs and commitments already in flight are debts to be honoured or renegotiated, not terms to be rewritten by fiat.

## Chains of succession: what bounds a line of successors

Debts are owed by the office, not by the holder. A debt is owed to a principal, by whoever holds the designer role, and when the role passes the creditor stays the same. The chain therefore neither multiplies the debt — the principal collects once — nor divides it: each holder owes the whole outstanding debt.

Transfer must be intact and itemized. Each handover's record must account for every outstanding debt as completed, renegotiated, declined, or passed on. A debt present in one link's record and absent from the next, with none of those four beside it, is default at that link.

Transfer renews no term. A debt's recorded term survives handover, and passing it on does not reset its clock or extend its deadline unless a design act revises them visibly. A chain that outlives a term without completion, renegotiation, or decline is default stretched across links, not continuation.

Legitimacy is not laundered. Each link's authority must trace to a prior core rule, and a break anywhere breaks everything downstream. A successor chosen by a clean procedure written by a capturer holds nothing, because no one can confer an authority they do not hold.

The chain itself can become the evasion. A debt handed on again and again, resetting nothing but the holder's name, is churn. Count handovers, or time without a debt completed, renegotiated, or declined, and when a designer-fixed threshold is crossed the chain flips to defective.

The chain ends in one of three ways, and only two are clean: debts paid, renegotiated, or declined, leaving the office to be succeeded empty or ended; or sunset, which does not erase debts — they must first be discharged or handed to a successor, or the sunset is abandonment, not ending. The third way is failure: the chain breaks, leaving a legible record of where it failed.

Corporate law makes the opposite default: a buyer of assets does not inherit the seller's debts, except by express assumption or mere continuation. A harness successor claims the office itself, so taking its name and authority without its debts is not a continuation.

## Identity across revision: the office, not the planks

A successor inherits the authority to revise the core, so how much may it revise before the harness is a different harness, not a successor? This is the ship of Theseus: planks replaced until none remain. For a harness, the code is the planks, but a harness is not its code, and its identity is not a property of its code at all.

Sameness of code or behaviour fails, because legitimate revision changes both. A similarity threshold fails worse: small revisions, each preserving identity, can walk the code to nothing in common with the original, so transitivity forces the threshold to permit total replacement or forbid every revision.

What survives is institutional continuity, through three traces: authority, with no gap in title; revision, with every core change a recorded, prospective, visible act and no retroactive erasure; and obligation, with the debt ledger intact and principals owed not shrunk silently. Where all three are intact, the harness is the same, however much code has changed; where any breaks, it is not. An exact copy of the code with no trace to the office is a clone, not a continuation.

Some rules are ordinary, and a successor may revise them extensively by ordinary design acts. Others are constitutive: they determine what counts as a valid design act, holder, or outstanding debt, including that acts be recorded, prospective, and visible, and roles stay separate. Call them recognition rules: the rule by which a system's rules are identified as its rules. A change to a recognition rule by an ordinary design act founds a new harness, however small the change.

Whether a rule is constitutive is functional, not a label: would changing it change how a later reader recognises a valid design act, holder, or outstanding debt? If yes, it is constitutive. If the core fixes a higher procedure for changing constitutive rules, a change following it preserves identity, as amendment, not replacement. If it fixes none, a constitutive change requires ending the old harness and founding a new one, with debts discharged or handed over.

A fork departing from the chain the core authorises is a founding, and takes debts only by explicit agreement, like a novation. A merger of two offices is likewise a new office. A recorded renaming preserves identity; a retained name after a break does not restore it.

## Trust in the records: provenance, evidence, and competing ledgers

A record is not a statement about the past. It is a statement with provenance: who wrote it, when, under what design act, chained to what came before. A bare assertion — "the debt was paid" — with no provenance is not a record. It is a claim, testable only by asking the claimant.

Correction must be distinguishable from forgery, and the distinction is in the form, not in the intent. A correction appends: a new record, visible to every principal, naming the record it corrects, the reason, and the design act behind it, leaving the old record in place. A forgery rewrites silently, with no new record marking the change. Append-only is therefore a condition of the identity test.

The store must sit outside the write set of the parties the records judge. A designer who can rewrite the record of their own design acts has no constraint. Each principal's view must therefore carry enough of the record to check it, so a rewrite must fool every viewer.

No store the designer controls can be proof against the designer. What it can be is evidentiary: each record commits to the one before it, so changing an early record invalidates every later record that commits to it. A forger must then rewrite the whole chain after it, and every copy held by another principal.

A chain held only inside the harness still has a forgery window. The window closes when a link is published to a store neither designer nor agent controls. An anchor does not make records true; it bounds the lie to before the anchor, or to rewriting the anchor in the open.

Trust is graded, not binary. A routine epoch completion may rest on a chained record in the harness views; a succession, constitutive change, or debt discharge should rest on records chained, independently held, and anchored. The core fixes what strength each kind of act requires, in advance.

Competing ledgers are adjudicated in the same terms. First, continuity: a ledger unbroken from the last record every principal's view agrees on, or from the last anchor, is the continuation; a ledger with a gap, or a link no other view corroborates, is a fork, taking debts only by explicit agreement. Second, if two ledgers both continue unbroken from the same anchor, the core's prior fork rule decides, if it has one, itself a recorded act. Third, if the core has no fork rule, the dispute is unresolved: suspend only the contested claims, let uncontested work proceed, and surface the dispute to the party who can fix a rule.

## Where the anchor regress stops: roots, not further records

Anchoring seems only to move trust one level out, since every store is a harness for someone else. It stops for three reasons. First, an anchor's claim is thin: it certifies not that a record is true, only that it existed, in a certain order, at a certain time. Existence and order are easy to check independently: many parties can hold the same link, and disagreement is visible without judging content. Content is judged inside, by provenance and continuity.

Second, the regress stops at the world boundary, drawn by the designer-control test: the first store no disputant controls, whose corruption would be visible to strangers. Inside, forgery is cheap and invisible; past that boundary, it must rewrite a store held by disinterested parties, in the open. No store is uncontrolled in every sense; the chain need climb only past parties with a stake in this claim.

Third, the regress stops at a root, and a root is not a record. A root is something a principal holds directly: a key, a copy of a chain, a witness known by acquaintance, an event observed firsthand. A root needs no provenance, because provenance is checked against roots. It is not proved; it is adopted by a design act fixed in advance — the core names the roots each kind of act requires — and its force is the principal's risk of being wrong. If it fails, that principal bears the loss and replaces it by a recorded design act, as with a compromised key.

A cycle grounds nothing: two harnesses anchoring each other's logs, or a chain looping back to a disputant-controlled store, is borrowed trust in a circle. The chain must reach a store outside every disputant's control, or a root held by the relying principal, before repeating a store, or contested claims stand suspended. Mutual anchoring is useful, but not grounding. The regress stops at a root adopted before the dispute, outside disputants' control, whose failure the relying party bears, because a party may not certify itself.

## Rotating a root without breaking the chain

A root is adopted, not proved, so a rotation is a new adoption, and the same office's root must be authorised by a rule fixed before the rotation, and leave past records unbroken — changing recognition rules by ordinary act founds a new harness.

Planned rotation has three acts, in order. First, the old root, while still trusted, vouches for the new one, by signing it or publishing it under its anchor. Second, both live together, the new visible before trusted, so transition forgery can be noticed. Third, the old root retires by a signed revocation naming its successor. Retirement is prospective only: the old root remains valid for past records.

Continuity is in the ledger of roots, not any single root: adoptions each vouched by a predecessor while still trusted, or by a reserve rule fixed in advance. A verifier holding any root can walk forward to the present root. A gap breaks the chain: a root adopted by ordinary act, or vouched by a compromised predecessor, is not the next root, and records under it stand suspended.

Emergency rotation tests the rule, because a compromised root cannot vouch for anything: a signature from a stolen key is exactly what the attacker can produce, so a successor vouched only by it is no successor. It must come from the prior rule: a reserve root named in the core, held apart until the emergency, a quorum of witnesses fixed in advance, or, if no uncompromised anchor remains, a fresh out-of-band design act, which re-founds rather than rotates. Then the new root's authority starts with its adoption, and old records keep only their own anchors.

Rotation is also an ending: announced, drained, archived. The overlap drains records in flight under the old root, so none is left judged under a root nobody can name. The signed revocation announces it to strangers; the retired root is archived, not deleted, so a later reader can name the root live at any record's time. Non-key roots rotate the same way, except a witness, copy, or habit cannot sign its successor, so rotation is by reserve rule or re-founding.

## The root ledger needs no anchor beyond its roots

What anchors the ledger of roots? Its anchors are the roots it lists, nothing above them, for three reasons.

First, an anchor above the ledger fails the regress section's own tests. It would be either a root — the very things the ledger lists, so a party would certify itself — or a further ledger, needing an anchor of its own, reopening the regress.

Second, the ledger's claim is thin, and checkable from inside: only the order of adoptions, each vouched by the root live at its time, the outgoing root's closing link naming its successor. Any verifier holding a root can check every later link, each vouch being verifiable under a root they can reach.

Third, the segment before a verifier's held root needs no anchor, because the held root is its anchor. A verifier holding R did not derive R from the ledger; they adopted R, bearing the risk of being wrong. Everything before R is grounded in that adoption; the ledger tells them only what happened after their root, which they can check.

Completeness — a deleted or inserted adoption — is the competing-ledgers case, settled by continuity and independent views. A ledger held by many cannot lose a link silently, because holders' copies disagree, visibly, without judging content. The more independent holders, the harder a silent rewrite. Where copies disagree and no prior rule settles them, adoptions after the break stand suspended, as do records judged under them.

A verifier holding no root cannot check a link, since every link is checked under some root. That is not a missing anchor; it is the regress-stopping act itself. Before checking, adopt, by a recorded design act from outside the ledger.

Test: walk forward from a root you hold, verifying each vouch under a reachable root, and comparing your copy with another holder's. If the walk completes and copies agree, the ledger is continuous from your root.

## The minimal core: what the theory commits to

The theory has grown by one move, repeated: answering a question one level up — steps to histories, harness to records, records to roots, roots to the ledger.

The core is small. It is these claims, and nothing else is load-bearing:

First, the boundary: agent, harness, world, split by who can change what, by the designer-control test.

Second, the state split: s = (w, h), world state uncontrolled by designer or agent, harness state designer-controlled.

Third, authorship: the agent may write harness state within a designer-permitted scope, shaping its future observations and transitions.

Fourth, views: one observation function per principal, satisfying sufficiency and separation — enough to do the job, never conferring a write by showing a read.

Fifth, arbitration: a designer-fixed rule on transitions, veto without compulsion, legible to the preempted principal, suspending what no precedence decides.

Sixth, liveness over histories: liveness is judged over traces, by counters and timers against designer-fixed thresholds, progress against aspiration, tied proxies, and epochs.

Seventh, the meta-rule: authority predates the act and sits outside the judged party; revision is a recorded, prospective, visible design act; uncovered cases freeze inherited values.

Everything after the meta-rule is application, not addition: succession to the designer role, authority passing only by a rule fixed before the absence; ending to the harness's lifetime, retirement as a design act, not a verdict; debts to obligations, what the office owes surviving the holder; identity to revision, the traces, not the code, being the harness; trust to the record, its force from provenance, chains, independent views, and roots adopted before the dispute.

A proposed addition earns its place only if it is a core claim, or an application of one to a new evasion, decided by the meta-rule. A claim true of institutions in general, but saying nothing about designer-controlled machinery, earns its place only if forced: without it, some failure cannot be attributed, some evasion cannot be closed, or some principal cannot be served. What makes a harness good, or what code looks like, is a next question, not a gap. Test: a claim neither in the core nor following from it is decoration; a core claim whose loss costs nothing does not belong.

## What each claim carries

Minimality is checked, not asserted: drop one claim, keep the other six, and ask what is lost.

Drop the boundary: failure loses its address, since a misleading prompt is simply outside the agent, so bad design cannot be told from an uncooperative world, nor agent self-editing from designer editing.

Drop the state split: a transition over an undivided state has no landing site, so editing a REPL variable and sending an email are both just acting, and authorship loses its object, since there is no h to write.

Drop authorship: the harness becomes a pipe, with h fixed by the designer alone, so the agent choosing what to see next, or installing a skill, has no description.

Drop views: with one observation function, the overseer either sees exactly what the agent sees, too little to judge or too much to be safe, or is served by no function at all.

Drop arbitration: when actions conflict, nothing decides except order of arrival, or the agent itself, which makes the decider a principal, and a conflict no rule covers proceeds silently instead of suspending.

Drop liveness over histories: judgement becomes step-local, so every step can be safe while the run never completes, or loops forever doing nothing forbidden, and vague goals become unjudgeable.

Drop the meta-rule: nothing stops the party they constrain from rewriting them, and an uncovered case is improvised rather than frozen, so the gap itself becomes the evasion.

## Membership: what counts as a harness, and what does not

Judge by function, not by product, part by part.

A candidate is an agent harness exactly when four things hold. First, a loop for one policy on one task, where the next step can depend on the previous observation. Second, designer-controlled machinery outside the policy's write set, except for a permitted authorship scope. Third, at least one action landing in the world or in harness state; reading is not acting, but reshaping the machinery the next observation comes from is. Fourth, a designer-fixed control altering or suspending execution without the policy's cooperation.

Weakness is not absence. A loop appending all history to the context, with no curation, is a bad harness, not a non-harness. A minimal loop that runs the test suite and calls a task done only if the suite passes is embryonic, because its check does not depend on the model's word.

The neighbours fail one of the four. A framework composes agents, but does not run their loops. An SDK offers loop primitives, not a running loop. A plugin suggests text without task state or verification. An orchestrator chains steps without observation-driven branching. An evaluation harness grades after the fact, without closing the inner loop for a single task. A runner launching a harness is not one. A bare prompt runs no loop.

Scaffold, workflow, application, and agent need the same test. A scaffold that runs the loop under designer-fixed controls is a harness under another name; starter code or wiring a developer completes is material, like an SDK — it offers parts of a loop and runs none. A workflow whose course no observation alters is not a loop; one that retries, replans, or branches on observation has a loop for that stretch and passes or fails as a loop does. An application, and an agent as a product, are bundles, not candidates: the test applies to the harness part of the bundle, not the bundle as a whole. A policy on its own is not a harness; a bundle with no loop, no designer control, or no control is not one either, however complete it looks as a product.

Two corrections are forced by the core. First, counting only world-altering actions is too narrow: RLM's REPL writes, notes, and skill installs alter no world state yet change the next observation; the landing site is h, not w. Second, a control counts by author: a rewritable cap or cooperative guardrail is a suggestion, not a control.

For any candidate, name the policy, loop, landing site, and designer-fixed control outside the policy's write set. If any cannot be named, the candidate is a neighbour, not a harness. Membership is binary; goodness is comparative.

## A worked example: SWE-agent

SWE-agent takes a GitHub issue and a repository at the buggy commit, and must produce a patch that fixes it (Yang et al., 2024, arXiv:2405.15793).

First, it runs a loop. `DefaultAgent.run`, in `sweagent/agent/agents.py`, is literally `while not step_output.done: step()`. Each step queries the model, parses thought and action, executes in a container, and saves the trajectory.

Second, the machinery is designer-controlled. The templates, tool bundles, history processors, cost limits, blocklist, and timeouts are configuration, in `config/default.yaml` and the agent and model configs, not model output.

Third, the policy acts on both sides of the split, though unevenly. World-directed: editing files, writing a reproduction script, running the test suite. Harness-directed, more thinly: every step writes, through the harness, its own trajectory, and that trajectory is what the next observation is built from.

Fourth, designer-fixed controls can alter or suspend execution without the policy's cooperation. Per-instance and total cost limits each end the run when exceeded. A command times out after 30 seconds, total execution after 1800 seconds, and repeated consecutive timeouts exit. A blocklist refuses interactive and escape commands; a blocked action is asked again, up to three times, for a different action.

A move's identity is its parsed intent and landing site, not its surface form. The moves are `bash` plus the configured bundles: a registry, an editor, and a review-on-submit bundle. Because `bash` is a move, `A(h)` is not a closed list: any shell string is a candidate move, minus the blocklist, minus remaining budget and time.

Observation is a function over the whole trajectory, not only over the container's current state. The default history processor is the identity, but the classic `LastNObservations` processor, keeping five observations, replaces older ones with `Old environment output: (n lines omitted)`. A designer-fixed processor thus rewrites history between steps, without any agent action. Only possible because `h` includes the history and trajectory lists, not just the container state.

Termination and verdict come apart. The loop terminates on submit, forfeit, cost, context, or timeout, or by autosubmission after an error, submitting the current diff. A run can therefore end as `submitted` without having succeeded. Success is judged outside the loop: the SWE-bench evaluation hook runs the maintainer's test suite afterwards and grades the patch, as a neighbouring evaluation harness. Inside the loop, the review-on-submit bundle checks the model's diff on submit.

What it lacks is as informative as what it has. There is no live view for a second principal: an overseer gets the trajectory file and inspector after the run, not an observation function of its own during it. A human model replaces the policy rather than intervening in a run already under way. It passes the membership test; passing is not goodness, since its open shell makes its control reactive rather than bounding.

## Human intervention in a run already under way

Basic question 10 asks how a human intervenes in a run already under way: pause it, edit it, redirect it, or take it over. Those four verbs differ in what they change and when they can take effect.

In SWE-agent there is no live intervention: a run is a batch loop, and a human's only way in is to replace the policy. That is not intervention in a run; it is a different run with the same loop.

OpenCode, as inspected in its current code (anomalyco/opencode, dev branch, commit 17b79f4e0c16dcff200b0417567ed22aeb2d7166), is the contrasting case. First, pause. `Session.interrupt` stops an active session, with the reason recorded as "user". It suspends transitions, rather than ending or judging the run. Interruption is acknowledged as soon as it is accepted, while cleanup settles asynchronously, and interrupting an idle session is a no-op.

Second, redirect while running. A new prompt does not rewrite the step in flight. It enters a session inbox as steered, for a step boundary mid-work, or queued, for the next turn. An interrupt with resume wakes the steering input, while queued prompts stay parked. A human cannot edit a step already executing, but can decide what the next boundary delivers.

Third, edit. Reverting a session to an earlier message, with files restored from snapshots, is staged and gated on idleness: stage, clear, and commit refuse with a busy error while an execution is active. The order is forced: pause first, then edit, then resume. An interrupt alone does not rewind landed world effects or restore files.

Fourth, replace the policy without replacing the run. Switching the agent or the model publishes a selection event, and the session, its history, and its harness state persist. That is takeover in the narrow sense: the policy changes, the run continues. It differs from SWE-agent's replacement, where a new run starts, and from a user taking over by typing the next step.

Two kinds of interruption must not be confused. An interrupted provider stream is recovered inside the step, with a synthetic observation telling the model to continue without repeating completed content. A user interruption aborts the step, fails it as "Step interrupted", fails unsettled tools as aborted, and injects no continue instruction. The next drain starts a new step, from the history, where completed tool results survive and aborted calls stand failed.

The theory takes for granted the user's write set: interrupt, steer, and revert, recorded with reason and author.

Settlement first. A session counts as active through interruption cleanup and terminal settlement. A second interrupt in that window is a no-op returning false, but it still claims the recorded wakes, so settlement starts no successor for a dead intent. Wakes arriving during cleanup are new admissions, and start a successor when the run settles. A caller needing settlement, rather than acceptance, uses `awaitIdle`, which waits through any successor too. A steer-scoped successor never promotes queued prompts.

Partial steps second. Resumption after a user interruption restarts the step, it does not continue it. The interrupted step is failed, its unsettled tools are failed as aborted, and its completed tool results stand in the history. The next drain runs a new step, with its step count reset, once a promoted input gives it something to do.

Precedence third. The code gives no rule ranking principals: precedence is decided per mechanism, and there is no overseer party to rank. An interrupt beats in-flight work unconditionally, but not the future: wakes arriving during cleanup start a successor at settle. Between the two interrupt reasons there is no ranking: the first interrupt sets the execution stopping, a second returns false, and arrival order decides that pair. An active execution beats an edit. At promotion, steered input beats queued input. A permission rule beats a single move without asking. A user reply beats all pending permission requests, since a reject fails the batch. An interrupt cancels a pending permission wait.

What is left open is the batch case. For a harness with no live view and no interrupt endpoint, intervention may collapse to starting a new run, and whether that counts as intervention, or only as replacement, is still open.

## The harness's error model: what fails, and who decides next

What happens when the model produces malformed output, refuses, loops, or calls a tool that fails or times out? "Error" is not one kind of event. Failures differ in whether a move was attempted, in who refused or failed it, and in whether the run continues, retries, or ends. They add no eighth core claim.

First, a failure to parse is not a failed action. A token sequence the parser cannot map to a move is not an action at all, yet the harness must respond: error observation, retry, or halt. That response is a harness transition rule, not an agent choice.

Second, a move refused before it is attempted is not a world failure. A blocklist or an approval mode that never asks reports that a designer-fixed control disabled the transition, not that the world refused. SWE-agent shows the shape: a blocked command is not executed, and the model is asked again, up to three times, for a different action.

Third, a move attempted and failed by the world is an observation about `w`, not a harness defect and not a control verdict. An observation that does not say whether the failure was a rule or the world leaves the policy unable to tell whether retrying the same move is sensible or futile.

Fourth, two things that look like errors are not step errors at all. A model refusal, emitted as text, is a parsed move, not a harness failure: the policy chose to say no, and the harness's question is what it does with that answer. A loop is fine at every step and defective only over the trace, judged by counters and timers over histories.

Fifth, the harness itself can fail. An interrupted provider stream in OpenCode is recovered inside the step, with a synthetic observation telling the model to continue from where it left off: the harness repairing its own observation pipeline, not the agent erring. A user interruption is the other side of the same coin: the step is failed as aborted, unsettled tools are failed as aborted, and completed tool results survive in history.

Sixth, resource exhaustion is a stop rule, not an error in the agent's sense. Cost, context, and timeout limits end a run by designer-fixed control. A single command timeout and a run-terminating timeout are different events: one is a failed move the policy may learn from, the other is the loop's stop rule firing.

The theory takes for granted that a failure is visible in its kind.

What is left open is considerable. Retry bounds are unexplained: why three requeries, and what happens at the bound, is unjudged. Timeout classification is unsettled: a slow tool may be a world failure or a harness clock, and the observation may need to tell them apart. Provider and tool errors may need different trajectory marks, and whether history processors preserve that mark is unexamined. Refusal handling has no general rule: a policy refusing its task differs from a tool refusing a command.

## Cost, budgets, latency, and permissions: where each lives

Basic question 9 asks what cost, latency, budgets, and permissions constrain, and where those constraints live. The four are not one thing: conflating them misattributes a veto as a price, a stop rule as an agent error, or a permission as a budget. This section sorts them, using only SWE-agent and Codex CLI.

Cost lives in the trace and in `h`, observed through a principal's view: the Codex CLI adapter gives the overseer cost records, while SWE-agent's overseer gets only the trajectory file after the run. Cost is a vector across levels, not a scalar attached to `A(h)`: tokens, counted after generation; a tool call, whose execution cost is unknown when chosen; and a world-directed effect, such as a message sent, which no token count measures. No move therefore has a single price the policy can weigh beforehand.

A budget is not a price. It is a designer-fixed cap in the core, a liveness threshold, and exceeding it ends the run; SWE-agent carries a per-instance cost limit and a total cost limit. A budget shapes `A(h)` from the outside, but it is not a move, and not a price the policy may trade against a larger payoff. By the author test, a policy-rewritable cap is a suggestion, and an agent-raised budget is evasion. Cost is measured; a budget is a norm about cost, fixed before the run.

Latency runs on two clocks, which the error model keeps apart: a per-move clock, under which a single command in SWE-agent times out after 30 seconds and fires a failed move the policy may learn from, and a per-run clock, under which total execution after 1800 seconds ends the run. Calling the first a run failure, or the second an agent error, misattributes both. Latency also enters liveness as a timer: how long since the run last made progress toward its aspiration. A slow tool and a stuck loop can look alike until the timer's crossing turns waiting into a verdict. 
Permissions shape `A(h)` directly, saying which moves the harness will attempt. Admission is a registry listing the moves that exist, and a move outside it is not attempted. Veto is a designer-fixed rule disabling an attempted move: SWE-agent's blocklist refuses known bad commands, and Codex CLI's forbidden exec-policy rules refuse theirs. Approval is a judging party deciding a request: Codex CLI's guardian or user, with a grant for a turn or session, never written to disk. Under default on-request approval, seeking is model-chosen, so sandbox and forbidden rules, not asking, hold regardless. A durable amendment differs from a grant: the grant changes attempts for a turn or session, the amendment changes the rule itself, a design act. Permission is neither arbitration nor a budget: arbitration decides between proposals, and a permitted move may be unaffordable while an affordable move may be forbidden.

Left open are whose view carries running cost, subagent attribution (an inner loop's cost is charged to the outer move that spawned it, but the summary hides the inner steps), latency as a progress proxy, and which permission changes may be granted mid-run.

## Do coding, browsing, chatting, and operating harnesses belong to one theory?

Basic question 11 asks whether coding, browsing, chatting, and operating harnesses belong to one theory, or whether the word "harness" covers different things in each case. This section considers the question; it does not resolve it. Every adapter so far is a coding harness, so the other kinds are treated as kinds, not verified systems.

The membership test is kind-blind, and that is a finding in favour of one theory. It asks for a loop, designer-controlled machinery outside the policy's write set, an action that lands somewhere, and a designer-fixed control that can alter or suspend execution without the policy's cooperation. None of those four mentions files, pages, messages, or clicks, so a loop that navigates pages, replies in a conversation, or drives a desktop can pass the same test SWE-agent passes, and a loop that fails any of the four fails it in any kind.

Passing the same test is not being the same thing. The core stays put, but three of its applications change shape by kind. First, the world and harness line, and who counts as a principal. In coding, the repository is world, and the trajectory, the templates, and the registry are harness state. In browsing, the web is world, but tabs, cookies, and a cached page are harness state, so cached divergence from the live world is the harness's failure. In operating, the desktop and its files are world, while focus, the clipboard, and the screenshot pipeline are harness state. Chatting is the sharpest case: a reply lands in the transcript in h, for another principal's view, and the interlocutor is a principal who can judge, interrupt, and end the run. Treating the interlocutor as world loses views, arbitration, and principal-judged progress; treating the interlocutor as a principal makes chatting multi-principal from the start, unlike single-policy SWE-agent. The kind changes the principal count, not the core.

Second, termination and verdict come apart differently. In coding, the loop ends on submit, and tests outside it judge success. In browsing, no submit exists: a loop may end at a page state, and a checker reading a changing world must say whether it reads the world or a harness snapshot. In operating, system state is both work and verdict, so stopping mimics success unless a check separate from the loop is defined. In chatting, a reply ends a turn, not a run; a run ends when the interlocutor ends it or a stop rule fires, and progress is principal satisfaction, not measured in w, with liveness over histories still applying and the aspiration held by a principal other than the designer.

Third, irreversibility, and the shape of the action set. In coding, open-shell moves are veto-based and often reversible by diff and revert, so review after the act can repair them. In browsing, links and buttons on the current page offer candidate moves, and A(h) lists attempts, not guarantees: the world offers, the harness attempts, the world determines success. In operating and chatting, moves are often irreversible — a message sent, a file deleted, a purchase placed — so control must be pre-emptive, by admission, veto, or approval before the move, not post-hoc review. Review-on-submit is therefore inadequate for those kinds, but membership requires only some designer-fixed control, not that shape; which shape is adequate is goodness, not membership.

Browsing strains the boundary in one further way, where a second theory would most plausibly be proposed. Page content is world, untrusted, and can contain instruction-like text; treating it as a designer instruction is misattribution, since the boundary is drawn by control and a page is not designer-controlled, so following it is a policy error or harness failure. A browsing harness must keep world content and designer instruction in different channels or views, or the boundary collapses in practice.

What would split the theory is a kind requiring a new core claim or systematically failing membership; the candidates do neither. Left open are notation (A(w, h) versus A(h) with the world's offer as observation), whether chatting progress needs its own verdict vocabulary, and a second, non-coding adoption — until which this section is prospective.

## Basic questions

These questions come before the more elaborate parts of
the theory. They do not require a resolved answer.
What a theory of harnesses owes each of them is
consideration: an explicit account of how the
question bears on the theory, what the theory
takes for granted about it, and where it leaves
the question open, even if no settling answer
is in sight.

1. What is the action space? Is an action a token, an
   API response, or a tool call, and how do those levels
   relate? Can the action space change as the harness
   state changes?
2. Is a partially observable Markov decision process the
   right mathematical framework? What does it capture,
   what does it distort, and what should complement or
   replace it?
3. What are the major harnesses used today, and what
   varies among them? Coding agents, frameworks for
   building agents, and research prototypes are not the
   same kind of thing.
4. What is a step, a turn, and a run? When does a run
   end, who decides, and what distinguishes stopping,
   failing, and being halted?
5. What is state, and how does it differ from context,
   memory, and the world? What is in `s`, and what is
   only text shown to the model?
6. What is a tool, as opposed to a subagent, a prompt,
   a skill, a memory, or a retrieval result? Are these
   different kinds of things, or the same thing under
   different names?
7. What is a harness, as opposed to a framework, a
   scaffold, an agent, an application, or a workflow?
8. What happens when the model produces malformed
   output, refuses, loops, or calls a tool that fails
   or times out? What is the harness's error model?
9. What do cost, latency, budgets, and permissions
   constrain, and where do those constraints live?
10. How can a human intervene in a run already under
    way: pause it, edit it, redirect it, or take over?
11. Do coding, browsing, chatting, and operating
    harnesses belong to one theory, or does the word
    "harness" cover different things in each case?

## Open questions

1. What is the minimal core that all harnesses share, without being
   so general that it says nothing? POMDP alone is too general to
   guide design.
2. What distinguishes a good harness from a merely working one,
   once cost, robustness, legibility to humans, and failure modes
   are included, not just reaching `G`?
3. Is agent control over observation the central principle, or one
   principle among several, alongside control over transitions and
   legibility?
4. How should a theory handle multiple principals at once: the
   agent, the user, and the human overseer, each needing their own
   observation function?
5. What invariants survive across good harnesses, across models,
   and across tasks?
6. How can a formalism be useful now and into the future, without
   knowing the future? What should it commit to, and what should
   it leave open, so it can be revised gracefully?
7. What would a harness look like if it were coded from a solid
   mental model, so cleanly that new developments fit into it
   without tangling, and maintenance stays low?

## Sources to read, test, and argue with

- Recursive Language Models, Zhang, Kraska, Khattab,
  arXiv:2512.24601, first posted December 2025.
- Prime Agent, Prime Intellect, August 2026, a self-improving
  harness on the RLM abstraction; §2.5's Continual Harness is the
  source of the fixed-core / mutable-supplement split.
- Classic reinforcement learning and decision theory: Sutton and
  Barto, and the older MDP / POMDP literature. Ch. 3's boundary rule
  is the source of the designer-control test.
- Supervisory control: Ramadge and Wonham, "Supervisory
  Control of a Class of Discrete Event Processes", SIAM
  Journal on Control and Optimization, 1987 — the veto-without-compulsion
  shape behind the arbitration section.
- Safety and liveness: Bowen Alpern and Fred B. Schneider,
  "Defining Liveness", Information Processing Letters
  21(4), October 1985 — the source of the liveness section's
  safety / liveness decomposition.
- Vague goals and aspiration: Herbert A. Simon, "A
  Behavioral Model of Rational Choice", Quarterly
  Journal of Economics, 1955. Satisficing is the source of the
  progress section's aspiration in place of a point.
- Succession fixed before vacancy, as illustration, not
  authority: the United States Presidential Succession Act of
  1947 and the Twenty-fifth Amendment, and the Linux kernel's
  succession and continuity plan — legitimacy from the rule
  fixed before the vacancy, not from the successor's claim.
- Software sunset practice, as precedent for announced, drained
  endings, not as authority: deprecation announced before shutdown,
  with a migration path and a date (IETF, RFC 8594, the Sunset
  header) — the source of the ending section's drain and
  announcement rules.
- Novation in contract law, as illustration, not authority — the
  source of the successor section's fourth inheritance.
- Trending and historical harness implementations on GitHub, to be
  read as examples, not as authorities. An implementation that
  works is evidence, not a theory.
- Corporate successor liability, as contrast, not authority: the
  traditional asset-buyer rule the chain section inverts, since
  a harness successor claims the office, not the assets.
- Identity across revision, as illustration, not authority:
  Plutarch, Life of Theseus, on the ship of Theseus, and Hobbes's
  second ship built from the discarded planks; and H. L. A. Hart,
  The Concept of Law, 1961, on the rule of recognition — the source
  of the identity section's recognition rules.
- Tamper-evident records, as illustration, not authority:
  hash-chained append-only logs and Merkle trees, as in
  version control and Certificate Transparency (IETF,
  RFC 6962) — the source of the trust section's graded trust
  and its bound on forgery.
- Trust roots, as illustration, not authority: a root
  certificate in public-key infrastructure is installed by
  the relying party, not derived from a chain — the source
  of the regress section's adopted root.
- Trust anchor rotation, as illustration, not authority:
  IETF, RFC 5011, "Automated Updates of DNS Security (DNSSEC)
  Trust Anchors", September 2007 — the source of the
  rotation section's overlap, closing link, and emergency rule.
- Agent harness definition, as foil and test, not authority:
  Sanderson Oliveira de Macedo, "What makes a harness a
  harness: necessary and sufficient conditions for an agent
  harness", arXiv:2606.10106, August 2026 — its four
  conditions and neighbour boundary are the membership
  section's foil and test, departed from in two places:
  harness-state actions count as acting, and a control
  counts only if designer-fixed and outside the policy's
  write set.
- SWE-agent, as worked example, not authority:
  John Yang et al., "SWE-agent: Agent-Computer
  Interfaces Enable Automated Software Engineering",
  arXiv:2405.15793, 2024, and the system's current
  code and default configuration — loop, action set, and
  controls are the worked example's source.
- Planning under partial observability, as the
  formalism judged, not authority: Leslie Pack
  Kaelbling, Michael L. Littman, and Anthony R.
  Cassandra, "Planning and acting in partially
  observable stochastic domains", Artificial
  Intelligence, 1998 — the source of the POMDP
  section's division of labour.
- Temporally extended actions, as complement,
  not authority: Richard S. Sutton, Doina Precup,
  and Satinder Singh, "Between MDPs and
  semi-MDPs: a framework for temporal
  abstraction in reinforcement learning",
  Artificial Intelligence, 1999 — the source of the POMDP
  section's treatment of a subagent move as a
  semi-Markov action at the outer level.
