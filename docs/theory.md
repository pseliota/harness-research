# Theory

This document states the definitions. It also states the questions that the definitions leave open.

## Definitions

- s stands for the pair (w, h).
- w stands for the world state, the part of s that neither the designer nor the agent controls.
- h stands for the harness state, the part of s that the designer controls.
- a stands for an action, one token or one parsed move at a stated level.
- o stands for an observation, the input at one step.
- pi stands for a policy, a function from an observation to a distribution over actions.
- Write pi(a | o) for the probability that the policy selects action a after observation o.
- p stands for a probability function.
- s' stands for the state after one step.
- Write p(s' | s, a) for the probability that action a in state s leads to state s'.
- Write p(o | s) for the probability that state s yields observation o.
- V stands for the model vocabulary, the set of tokens the model outputs.
- A stands for an action set at one level of abstraction.
- A1 stands for the action set at the next level, the set of parsed moves.
- A(h) stands for the set of moves available to attempt in harness state h.
- G stands for the set of goals for one run.
- A principal is a party with its own goal, observation function, and write permission set, such as the user, the agent, or the overseer.
- p_i(o_i | s) stands for the observation function for principal i, where o_i is the observation of principal i.
- G_i stands for the version of G in epoch i. An epoch is a period in which the harness functions stay fixed and G stays at one version.
- An index letter ranges over the set named with it. In G_i, i ranges over epochs. In p_i, i ranges over principals.

The harness is part of the environment. The harness carries out the transition function and the observation function.

A tool gives the agent an answer without work by the agent. The harness calls an outside service and puts the answer in the next observation. For weather, the harness calls a weather service and puts the result in the next observation.

## Harness boundary by control

Sutton and Barto draw the boundary by control. What the agent does not control counts as environment. The same rule splits the environment three ways, by who controls what.

- The agent side is the policy and, for a language model, its weights.
- The harness side is the transition and observation machinery, internal state, interfaces, budgets, and checks.
- The world side is weather, external service answers, and retrieved documents.

The system prompt is harness, not agent. The designer rewrites it and leaves the weights unchanged.

Apply this test: the designer changes a part, and the agent and the world stay fixed. If the part changes, it is harness. If a change to the weights changes the part, it is agent side. If a change to the world changes the part, it is world side.

The label "harness" does not mean all outside the agent. That broad label merges the designer lever with the world. The merge hides who caused a failure, the designer or the world.

## Harnesses that rewrite themselves: fixed core and mutable supplement

Prime Agent Continual Harness rewrites its own prompts, memories, and skills. This fact looks like a counterexample to the boundary by control. It is not a counterexample at the right level. Split the harness into a fixed core and a mutable supplement. The core is the base prompt, the runtime that applies edits at turn boundaries, and a version record that records provenance and permits rollback. The supplement is prompt notes, memories, skills, and subagent specifications.

The agent edits the supplement through ordinary actions, under a modification policy from the designer. The policy states what the agent edits, when it edits, on what evidence, and with what provenance. The test survives one level up: the designer controls the core and the policy, not the supplement.

A bad skill from the agent is agent side. A bad core, a permissive policy, or a failed rollback is designer side. This attribution holds exactly when the modification machinery sits outside the write set of the agent. The write set is the set of state components the agent has write permission for. Prime Agent keeps its runtime outside the writable state.

This document does not cover a harness that learns its own modification policy from experience.

## Action space at levels of abstraction

No single action set serves every level of abstraction. Each level has its own action set, with the same shape for one step. An executed action at a higher level unfolds into a run at the level below.

At the lowest level, the action set is V. The policy pi(a | o) is a distribution over V, and nothing else is an action at this level. At the next level, the harness groups token sequences into parsed moves in A1. The parsed moves are emit text, call a tool, spawn a subagent, or stop. A1 is a quotient of token sequences: two spellings of one tool call count as one move. The same text is a different move under a different parser or registry.

A move has two parts: its parsed intent and the state component it changes. Two moves with the same intent but different state components are different moves. A move changes w, h, or both. A tool move takes one harness step. A subagent move runs a whole inner loop and returns only a summary. The inner loop is itself a harness.

The action set depends on state: write A(h), not A. Registry, access control, skills, and a budget cap in h add or remove moves. A new skill adds moves, a lost tool removes moves, and an exhausted budget cap leaves no costly move. A(h) lists moves available to attempt, not guarantees. Success still depends on w.

Skills, prompts, and memories are not moves. They are harness state that changes later moves and later observations. A skill changes moves and the parser. A prompt shapes the next observation. A note changes a later observation. A skill installed now changes A(h) later.

A retrieval result is an observation, not an action. A chat reply lands in the transcript in h, for another principal to see. The state component and the audience differ in that case. Unparseable token sequences are no action at all, not failed actions. The harness still responds, with an error, a retry, or a halt. That response follows a transition rule in the error model, not a rule in the action space.

Invert the cut: the harness code is the policy, and the model is the environment. The same levels of abstraction apply, and the two roles exchange places.

## POMDP as the shape of one step

For one step, a POMDP fits. For a harness, it does not fit. A POMDP is the shape of a step, not the theory of the steps. The agent never sees w directly, and much of what it sees is designed h. But a POMDP is one tuple: one state set, one action set, one observation set, one transition function, one observation function, and one reward.

1. First, a harness has not one action set. The levels of abstraction give a family of POMDPs, one per level, and the parser and expansion link the levels. A rejected token sequence is in neither action set. A subagent move spans many steps, so it is not a one-step move.
2. Second, a harness has not one observation function. Each principal has its own p(o | s) over the same s. The user, the designer, and a subagent see different observations of the same state.
3. Third, the functions of a POMDP stay fixed, but the functions of a harness do not. An epoch is a period in which the harness code stays fixed. Within one epoch, a run is a POMDP, and across epochs, a run is a family of POMDPs, one for each recorded design decision. Registry, parser, and thresholds change by later design decisions. A blocklist is a veto, not a price: it forbids a move, it does not add a cost. G moves across epochs, with a revision record for each change.
4. Fourth, the observations of a POMDP are evidence, but the observations of a harness are designed artefacts. SWE-agent history processor replaces older observations with an omission note. The next observation comes from the rewritten trajectory. This fit holds only because trajectory lists sit in h. A belief from a designed record is a belief about what the designer chose to show, not about w.
5. Fifth, the judgement of a POMDP is a step reward, but the judgement of a harness is partly over traces. Liveness is a whole-history property: deadlock, starvation, livelock, and progress against G. A step reward looks at one step only, so it misses a deadlock that spans many steps. A step reward encodes a trace judgement only if the trace enters the reward.

Finally, the vacuity point: if s and o each hold the whole history, any process is Markov. Then the claim "a run is a POMDP" is true and empty. The claim gains content only for a fixed s. For a fixed s, state which POMDP, which level, which epoch, which principal, and which vetoes apply. Keep the POMDP as local shape, for one level and one epoch. The rest of the theory goes around it.
## Goals

G can contain task goals and goals outside task performance.
For example, a human checks system health in a run.

Fix the agent.
Vary the harness.
The harness change makes G easier to reach.
People find G hard to specify in full.
People change goals, and a run must accept a revised goal.

## Example: Recursive Language Model

In a Recursive Language Model, a long prompt lives as one variable in a Python REPL.
The root model sees only metadata about the prompt.
The root model writes code to inspect the prompt and split the prompt into parts.
The root model calls sub-models on the parts.
Python does not create the power.

The agent gains control over its next observation.
The agent explores.
The agent acts on what it learns.
The agent explores again.

The agent can select only evidence that supports one branch.
The agent can miss evidence against one branch.
The agent can stay in one plausible branch.
The agent can miss a better branch.

The designer must decide which observation actions exist.
The designer must decide what each observation action costs.
The designer must decide which check requires a second look.

## World-directed actions and harness-directed actions

w holds facts outside designer control, such as tool results and external messages.
h holds facts the designer controls, such as prompts, memory entries, and plans.

o can hold text, a structured tool result, or system metrics.


A world-directed action changes w.
For example, an agent sends a message.
For example, an agent places an order.

A harness-directed action changes h.
For example, an agent writes a REPL variable.
For example, an agent writes a memory entry.
For example, an agent writes a plan.

The harness gives the agent partial write permission over h.
Write permission states which state components the agent may change.
A write outside the permission set fails.
A change to h changes the next observation.
A change to h changes the next available action.
In RLM, a write to a REPL variable shapes the next observation.

SWE-agent shows the same split.
An edit to program code changes w.
An edit to a plan changes h.

The designer must set the write permission.
The designer must set the cost of each write.
The designer must set the check on each write.
The designer must record the source of each write.

The fixed core is the part of h that the agent cannot revise.
The agent cannot revise a fixed core check.

The split depends on the boundary the designer draws.
The same file counts as h if the designer controls the file.
The same file counts as w if an external party controls the file.

One principle does not cover all harness questions.
Write permission explains control over future observations and transitions.
Write permission does not decide what another principal sees.
Write permission does not decide conflict between principals.

## Multiple principals and one state

A principal is a party with one goal, one observation function, and one write permission set.

The modification policy states which changes to h the overseer allows.

The agent observes the context history.
The agent writes h within its write permission.
The agent pursues G.

The user observes an answer or a summary.
The user writes instructions that set or revise G.
The user judges whether the answer serves the user.

The overseer observes the trace, the costs, the checks, and system health.
The overseer writes the fixed core and the modification policy.
The overseer can halt a run and roll back a change.
The overseer pursues a goal outside the task.

In one weather case, one tool call in s yields three observations.
The agent gets the structured result.
The user gets "sunny, 21°".
The overseer gets latency, cost, and success of the call.

Sufficiency means the observation gives the principal enough to do the job.
If the agent observation lacks the tool result, the agent cannot act on the result.
Separation means a read does not confer a write.
If a user read of a plan grants a write to the plan, separation fails.
A read of one state component must not change write permission for that component.
A write to h changes what each principal sees next.

The designer must decide in advance which agent writes each principal sees.
A change to the modification policy changes what principals see.

## Conflict between principals: precedence rules in the harness

When goals disagree, the harness decides.
The designer fixes the precedence rule in advance.
The precedence rule is a recorded design decision in the harness.
A decider with its own goal counts as one more principal.
The harness applies the fixed rule at run time.
The harness must not invent a new precedence rule at run time.

At one step, principals propose actions.
The precedence rule enables some proposed actions for that step.
The precedence rule disables the rest for that step.
Precedence works as veto without compulsion.
The harness can disable one proposed action.
The harness cannot force the agent to take a different action.

Ramadge and Wonham describe a supervisor with the same limit.
The supervisor disables some controllable events.
The supervisor cannot force the plant to generate one event.

Precedence does not rank principals in one total order.
A user revision of G outranks agent continuation of the old goal.
A fixed core check outranks user urgency.

If two proposed actions conflict and no fixed precedence decides between them, the harness suspends only the contested action.
Unrelated work proceeds, so one conflict does not block all progress.
The harness surfaces the conflict to the designer through the overseer.

An overseer that halts at each step blocks each user goal.
Each run terminates with no answer and a halt record.
The user can check the failure from the halt record.
The user cannot obtain the answer from a halted run.

Preemption must show in the observation of the preempted principal.
The observation must name the rule behind the preemption.
The observation must name the suspended action.
If the observation lacks the rule and the action, the preempted principal cannot check why the old goal continued.

For example, the harness suspends a user revision of G.
The user sees only the old answer.
The user cannot tell that a fixed core check blocked the revision.
## Safety and liveness under a precedence rule

Alpern and Schneider (1985) show that each property of a run history combines a safety property and a liveness property. Safety is step-local. A safety property states that nothing bad happens. A violation of a safety property already shows in a finite prefix of the run history.

Liveness is a property of a whole run history. A liveness property states that something good eventually happens. Each finite prefix of a run history can still extend to satisfy a liveness property.

Deadlock means that no principal can act. Starvation means that the actions of one principal never take effect while other principals act. Livelock means that actions occur but the run makes no progress toward G. Ramadge and Wonham require a supervisor that leaves a path to completion from each reachable state.

A precedence rule states which principal acts first when two principals seek the same action. Preemption occurs when a precedence rule blocks an action. A precedence rule that always vetoes the actions of one principal causes starvation. The run history shows starvation when the actions of that principal never take effect. A fairness condition bounds how long a principal waits before it acts. Round-robin, a time limit on delay, or an explicit override fixes starvation.

A counter counts consecutive preemptions of one action. A timer measures the time since the run last made progress toward G. The designer fixes what counts as progress and what threshold marks a defect. When a counter or a timer crosses the threshold, the run becomes defective. A defective run is a run that fails the liveness condition.

The liveness verdict is a change in run status, not an action that competes for precedence. The harness computes the verdict from the run history. The overseer then aborts the run, escalates to the designer, or relaxes the check. The overseer takes only actions that the designer permits. The observation function of a preempted principal must include the fact of preemption and the rule that fired. If it does not include them, the principal cannot distinguish preemption from its own failure.

## Progress toward a vague or revised goal set: epochs, satisficing levels, and proxies

G is often vague. G often shifts. Simon (1955) defines a satisficing level as a level that a principal judges as good enough. Search stops at the first state that clears the satisficing level, not at an optimum. The principal judges progress on criteria that the principal can observe. Progress is approach to the current satisficing level, not distance to a fixed point.

A proxy is a substitute measure for progress. New information gained and uncertainty reduced are examples of a proxy. A proxy is legitimate only when it ties back to G. The tie means that a principal can use the proxy to judge G or to advance toward G. If a principal optimizes the proxy instead of G, the proxy fails. For example, the run can gain information forever and never reach G.

Only a principal with write permission over G may revise G. That principal is usually the user. Each revision starts a new epoch. The revisions form the sequence G_0, G_1, ... Counters and timers run within one epoch. Counters and timers reset only at an epoch boundary that the harness records with provenance.

A revision that redefines G to a state already reached does not count as progress in the old epoch. If the principal that revises G accepts the current state under the new goal, the run completes under the new epoch. The harness records that completion as a revision, not as progress. A silent redefinition that resets timers without a recorded epoch boundary is evasion, not an epoch.

Churn means that revisions repeat and each revision resets the timers. In churn, no epoch completes and no epoch becomes defective. The harness counts revisions and time without a completed epoch. When that count crosses a threshold that the designer fixes, the run becomes defective. The rule that defines progress sits in the fixed core.

## Thresholds without a designer: rules that govern design decisions, values fixed in advance, and succession

Thresholds in the fixed core persist as inheritance. They stay in force until a legitimate revision changes them. The agent must not set its own thresholds. The overseer must not set thresholds at runtime in its role as judge. The harness must not learn thresholds from its own run histories.

A threshold is normative. It states what must count as a defect, not what usually happens.

The designer fixes a rule that governs design decisions. That rule states how the harness computes a threshold value, from what inputs, and within what bounds. The quantity that the rule judges must not set its own threshold. For example, a delay threshold must not grow because delays grow longer. The quantity must not scale with the trouble itself.

Where no rule that governs design decisions covers the case, the default is to keep the last values, not to relax them. A later revision is possible only as a recorded design decision. The harness records a threshold change with provenance, and the observation of each principal includes it. A threshold change applies only to future run histories. It does not erase a prior verdict or clear the run history that produced it. The observation of each principal must show that thresholds come from inheritance and that the designer is not currently reachable.

## Succession: who inherits the designer role, and on what authority

Succession is the transfer of the designer role. The designer role is the authority to revise the fixed core. Those rules include the precedence rules, the progress rule, the thresholds, and the rules that govern design decisions. Succession is not an action that competes for precedence. The precedence rule does not decide succession, because it gates actions. A liveness verdict does not decide succession, because it judges run histories.

The succession rule must already sit in the fixed core before the absence it governs. In the first legitimate form, the fixed core name a person, a role, or an ordered list. Authority passes down the list when the trigger condition holds. In the second legitimate form, the fixed core name no person but fix how the harness chooses a successor. That procedure states who initiates the choice, who meets, within what time, and by what vote or confirmation. The successor that the procedure chooses inherits the role.

Self-appointment is evasion. A principal that declares itself designer has no prior rule behind it. The agent cannot succeed the designer. Succession by the agent puts the judged principal in charge of its own threshold.

The overseer cannot promote itself from judge to designer in order to escape a verdict. Promotion by the overseer lets the judge escape its own verdict. A user majority at runtime cannot confer the role unless a fixed procedure states that a vote confers it. Then the procedure, not the majority, confers it.

A legitimate successor inherits the designer role, not only its powers.

- The successor inherits the fixed core as they stand, with the rules that govern design decisions and the values fixed in advance.
- The successor inherits the records, the provenance, and the prior verdicts, and it cannot erase them or clear them retroactively.
- The successor inherits the constraints on design decisions, and the harness records each decision, applies it prospectively, and shows it to every principal.

A delegate acts within a mandate that the designer still holds and can revoke. A successor replaces the designer, and the old designer cannot revoke the succession.

Succession also needs a trigger condition in the fixed core. Without that condition, succession is usurpation with a name attached. Once the trigger condition holds, the transfer must be legible. The observation of each principal must show that the designer is absent and that the trigger condition holds. It must also show that the successor now holds the role under the prior rule.

If no successor exists, runs complete or suspend under the values fixed in advance, and the vacancy stays visible. The vacancy does not pass to the principal that acts first.

## Retirement: when a harness may properly retire, and how it retires well

Retirement is not the same as termination of a run. A run terminates. A harness retires. In retirement, the harness accepts no new runs, closes open runs, and hands on or discharges responsibilities.

Retirement is a recorded design decision, not a verdict. The designer role decides retirement, under the constraints on design decisions. The harness records each retirement decision, applies it prospectively, and shows it to every principal. Only the designer or a legitimate successor under the succession rule may retire the harness. A harness that simply stops, with no record of who decided or why, is abandoned, not retired.

Three legitimate retirements exist, and they differ in shape, not only in name.

- In completion of purpose, the epochs of G complete and no new epoch opens.
- In supersession, a new harness takes over the purpose, and it receives the fixed core, the records, and the constraints on design decisions.
- In sunset, the designer or a successor judges that the purpose is no longer worth the cost of readiness. That judgement is a recorded design decision with reasons, like a threshold revision.

The designer or a successor announces retirement in advance, with a date, before it takes effect. The announcement lets principals finish their epochs or move. The harness accepts no new runs. It lets open runs complete or checkpoint at a clean boundary. It stops only after open runs reach that boundary. The harness archives the records and does not erase them.

## Definitions for succession and trust

An epoch is a period with fixed harness functions and goals. Outstanding obligations are obligations that an old harness still owes at handover. The designer role is the role that holds design authority for a harness.

A recorded design decision is a decision that the harness records before it takes effect and that stays visible to principals. Provenance is the record of who wrote a record, when, and under which recorded design decision. A trust anchor is a key, a copy, or a witness that a principal holds directly. It is not a further record.

## What a successor owes: outstanding obligations

In supersession, a successor takes over from an old harness. The successor inherits the core, the records, and the constraints. It also inherits outstanding obligations. A successor that takes authority and disowns outstanding obligations uses supersession to default.

An obligation transfers only if it appears in the record.

- An in-flight epoch is an epoch that starts under the old harness and still runs at handover.
- A recorded commitment is a commitment that the old harness records with a stated term.
- A promised checkpoint or handover is a checkpoint or handover that the old harness promises to a principal.

For an in-flight epoch, the successor completes the epoch under the old terms. Alternatively, the successor brings the epoch to a clean checkpoint and announces the transfer. A recorded design decision may revise the terms, but the revision stays prospective and visible.

For a recorded commitment, the successor honours the commitment until its recorded term. Alternatively, the successor renegotiates the commitment.

Transfer adds no new cost, no new visibility, and no new condition to an in-flight epoch. The successor gives notice and renegotiates before it adds a burden of that kind.

The successor may set new terms for new epochs. That power is its design authority.

## Chains of succession: limits on a line of successors

An outstanding obligation is owed by whoever holds the designer role. The creditor stays the same when the role passes. The principal collects once. Each holder owes the whole outstanding amount.

Each handover record accounts for every outstanding obligation as completed, renegotiated, declined, or passed on. An obligation with none of those four marks in the next record is a default at that link.

A recorded term survives handover. Handover resets no clock and extends no deadline. A recorded design decision may revise the term, but the revision stays prospective and visible. A chain that outlives a term without completion, renegotiation, or decline is a default stretched across links.

Each link in the chain traces its authority to a prior core rule. A break at any link breaks every link after it. A successor chosen through a procedure that a party without authority wrote holds no authority.

Repeated handover with no completion, no renegotiation, and no decline can hide a default. The designer role fixes a threshold in advance for handovers or for time without a decision on an obligation. When the chain crosses that threshold, the chain is defective.

The chain ends cleanly when the successor pays, renegotiates, or declines every outstanding obligation. Then the designer role stands empty or ends. Sunset is also clean only after the old harness discharges every outstanding obligation or hands it to a successor. Without that discharge or handover, sunset is abandonment, not an end. If the chain breaks, the record shows where it failed.

In corporate law, an asset buyer takes no seller obligation without express assumption. A harness successor claims the designer role itself. A successor that takes the name and the authority without the outstanding obligations is not a continuation.

## Identity across revision: the designer role, not the code

The ship of Theseus replaces each plank until no original plank remains. A harness is not its code, and code sameness cannot test identity. Legitimate revision changes code and behavior, so sameness fails as a test. Small revisions can each preserve identity and still walk the code to a state with nothing in common with the original. A similarity threshold therefore permits total replacement or forbids every revision.

Identity rests on three traces.

- The authority trace shows no gap in title.
- The revision trace shows every core change as a recorded decision that stays prospective and visible.
- The revision trace shows no retroactive erasure.
- The obligation trace shows the obligation record intact.
- The obligation trace shows that principals lose nothing silently.

Where all three traces stay intact, the harness stays the same, however much code changes. Where any trace breaks, the harness is not the same. An exact copy of the code with no chain to the designer role is a clone, not a continuation.

Some rules are ordinary, and a successor may revise them extensively by recorded design decision. Other rules are constitutive. A constitutive rule determines what counts as a valid design decision, holder, or outstanding obligation. Constitutive rules include the rules that design decisions stay recorded, prospective, and visible, and that roles stay separate.

The test for a constitutive rule is functional, not a label. Ask whether a change alters how a later reader recognizes a valid design decision, holder, or outstanding obligation. If the answer is yes, the rule is constitutive.

A recorded design decision under the ordinary procedure for a constitutive change founds a new harness, however small the change. If the core fixes a higher procedure for a constitutive change, a change under that procedure preserves identity as an amendment, not a replacement. If the core fixes no higher procedure, a constitutive change requires an end to the old harness and a new harness. In that case, the old harness discharges outstanding obligations or hands them to a successor.

A fork that departs from the chain the core authorizes is a new harness. A fork takes outstanding obligations only by explicit agreement. A merger of two designer roles is a new designer role. A recorded name change preserves identity. A retained name after a break does not restore identity.

## Trust in records: provenance, evidence, and conflicting logs

A record without provenance is a claim, not a record. A bare assertion that an obligation was paid is testable only by a question to the claimant.

A correction appends a new record that stays visible to every principal. The new record names the record it corrects, the reason, and the recorded design decision behind it. The old record stays in place. A silent rewrite with no new record for the change is forgery. Form distinguishes correction from forgery, not intent.

Without append-only records, the revision trace fails. The log must sit outside the write set of the parties that the records judge. A designer role that can rewrite the record of its own design decisions faces no constraint. Each principal holds enough of the log to check it. A rewrite must fool every principal copy.

No log that the designer role controls proves the log against the designer role. Each record commits to the record before it. A change to an early record invalidates every later record that commits to it. A forger then rewrites the whole chain after it and every copy that another principal holds.

A chain held only inside the harness still leaves a forgery window. The window closes when a link reaches a log that neither the designer role nor the agent controls. A trust anchor does not make records true. It bounds the lie to before the trust anchor, or to a rewrite of the trust anchor in the open.

A routine epoch completion may rest on a chained record in principal copies. A succession, a constitutive change, or an obligation discharge needs chained records, independent copies, and a link to a trust anchor. The core fixes in advance what strength each kind of act requires.

Conflicting logs follow three steps in order.

- First, continuity decides: a log unbroken from the last agreed record or trust anchor is the continuation.
- A log with a gap, or with a link that no other copy corroborates, is a fork.
- A fork takes outstanding obligations only by explicit agreement.
- Second, if two logs both continue unbroken from the same trust anchor, a fork rule recorded in advance decides, if the core has one.
- Third, if the core has no fork rule, the dispute stays unresolved.
- Suspend only the contested claims, let uncontested work proceed, and surface the dispute to the party that can fix a rule.

## Where the trust regress stops: trust anchors, not further records

A trust anchor certifies only existence, order, and time, not truth. Many parties can hold the same link, and disagreement stays visible without a judgment on content. Provenance and continuity judge content inside.

The world boundary follows the designer-control test: the first log that no disputant controls, whose corruption stays visible to strangers. Inside that boundary, forgery stays cheap and invisible. Past that boundary, a forger must rewrite a log in the open, against disinterested holders. No log is uncontrolled in every sense. The chain climbs only past parties with a stake in this claim.

A trust anchor needs no provenance, because a checker tests provenance against trust anchors. No one proves a trust anchor. The core names the trust anchors that each kind of act requires, and a recorded design decision adopts them in advance. The force of a trust anchor is the risk that the party that relies on it bears if it is wrong. If a trust anchor fails, that party bears the loss and replaces it by a recorded design decision. A compromised key is an example.

A cycle grounds nothing. Two harnesses that anchor the logs of each other form borrowed trust in a circle. A chain that loops back to a disputant-controlled log does the same.

The chain must reach a log outside every disputant control, or a trust anchor that the principal holds, before it repeats a log. Otherwise, contested claims stand suspended. Mutual links are useful, but they do not ground a claim. A party does not certify itself.

## Rotation of a trust anchor without a break in the chain

Rotation is a new adoption. The same designer role authorizes the new trust anchor by a rule fixed before the rotation. Rotation leaves past records unbroken. A constitutive change by an ordinary procedure founds a new harness.

Planned rotation has three acts, in order.

- First, the old trust anchor, while it stays trusted, signs the new trust anchor or publishes it under its trust anchor.
- Second, both trust anchors live together, the new trust anchor stays visible before it gains trust, and viewers notice transition forgery.
- Third, the old trust anchor retires by a signed revocation that names its successor.
- Retirement stays prospective only: the old trust anchor stays valid for past records.

Continuity sits in the trust anchor ledger, not in any single trust anchor. A predecessor vouches for each adoption while that predecessor stays trusted, or a reserve rule fixed in advance authorizes it. A verifier that holds any trust anchor can walk forward to the present trust anchor.

A gap breaks the chain. A trust anchor adopted under the ordinary procedure is not the next trust anchor. A trust anchor vouched for only by a compromised predecessor is not the next trust anchor. Records under such a trust anchor stand suspended.

A compromised trust anchor cannot vouch for a successor. A signature from a stolen key is exactly what an attacker can produce. A successor vouched for only by a compromised trust anchor is no successor. The successor comes from a prior rule.

- A reserve trust anchor named in the core, held apart until the emergency, can serve as successor.
- A quorum of witnesses fixed in advance can serve as successor.
- If no uncompromised trust anchor remains, a recorded design decision from outside the core re-founds the harness rather than rotates a trust anchor.

In that case, the new trust anchor gains authority at its adoption. Old records keep only their own trust anchors.

Rotation also ends a trust anchor: announced, drained, archived. The overlap drains records in flight under the old trust anchor. No record then stays under a trust anchor that no one can name. The signed revocation announces the retirement to strangers. The retired trust anchor stays archived, not deleted, so a later reader can name the trust anchor live at any record time.

Non-key trust anchors rotate the same way. A witness, a copy, or a habit cannot sign a successor. For those trust anchors, rotation comes by a reserve rule or by a new start.

## The trust anchor ledger needs no trust anchor beyond its trust anchors

The trust anchor ledger is the ordered list of trust anchor adoptions. Its trust anchors are the trust anchors that it lists, and nothing stands above them.

A trust anchor above the ledger fails the tests in the trust regress section. It is a trust anchor, the very thing that the ledger lists, so a party certifies itself. Alternatively, it is a further ledger that needs its own trust anchor, and the regress reopens.

The ledger claim stays thin and checkable from inside. It covers only the order of adoptions. The trust anchor live at its time vouches for each adoption. The last trust anchor closes with a link that names its successor. Any verifier that holds a trust anchor can check every later link under a trust anchor that the verifier can reach.

The segment before the trust anchor that a verifier holds needs no further trust anchor. A verifier did not derive that trust anchor from the ledger. The verifier adopted that trust anchor and bears the risk of a wrong adoption. Everything before that trust anchor is grounded in that adoption. The ledger tells the verifier only what happened after that trust anchor, and the verifier can check that part.

Completeness covers a deleted or inserted adoption, and it follows the conflicting logs case. Many holders prevent a silent loss of a link, because holder copies disagree visibly, without a judgment on content. More independent holders make a silent rewrite harder. Where copies disagree and no prior rule settles them, adoptions after the break stand suspended, as do records judged under them.

A verifier that holds no trust anchor cannot check a link, because a verifier checks every link under some trust anchor. That gap shows no trust anchor. It is the act that stops the regress itself. Before a check, the verifier adopts a trust anchor by a recorded design decision from outside the ledger.

Test: walk forward from a trust anchor you hold, verify each vouch under a reachable trust anchor, and compare your copy with another holder copy. If the walk completes and copies agree, the ledger is continuous from your trust anchor.
## The minimal core

Seven claims form the core. Later sections apply these claims. They add no new core claim.

1. Boundary. The boundary separates agent, harness, and world by write permission. The designer controls the harness. The agent controls its own moves. Neither the designer nor the agent controls the world.
2. State split. The state split separates every state into w and h. A move changes w, h, or both, and the split names which part changes.
3. Write permission. The agent holds write permission for part of h inside a scope the designer permits. That permission shapes later observations and transitions.
4. Observation functions. Each principal i receives data through p_i. The function gives enough data for the job of principal i. The function gives no write permission through shown data.
5. Precedence rule. A designer-fixed precedence rule decides conflicts between actions. Veto power blocks an action and never forces one. The rule shows its decision to the principal it preempts. The rule suspends an action that no precedence decides.
6. Progress and fairness. Check progress and fairness over run histories, not over single steps. Counters and timers measure events and time against thresholds the designer fixes. Progress tests use goals in G, tied proxies, and epochs. A tied proxy is a measure tied to a goal in G. An epoch is one period in a run history.
7. Rule that governs design decisions. Authority for a rule predates the act the rule judges. Authority sits outside the party the rule judges. Revision is a recorded design decision, visible to principals, for later acts only. An uncovered case keeps values from earlier rules.

The core also carries five applications. Succession passes authority only by a rule fixed before the absence. A harness retires only by a recorded design decision, not by a verdict on a run. A verdict on one run never retires the harness.

Obligations of the designer role survive a change of holder. Traces, not code, identify the harness after revision. A record earns trust from its provenance and from independent observation functions.

A proposed claim joins the core only when it passes one test. Drop the claim and keep the core. Ask whether a failure then lacks an address, an evasion then stays open, or a principal then lacks service. Admit the claim only when the answer to one question is yes. A claim true of institutions in general fails the test when it says nothing about designer-controlled machinery. Questions about what makes a harness good, or what code looks like, fail the test.

## What each core claim commits to

Drop the boundary. Bad design then looks like an uncooperative world. An agent edit to itself then looks like a designer edit.

Drop the state split. A REPL variable edit and an email then count as the same kind of act. Write permission then has no separate part to target.

Drop write permission. The designer alone then fixes h. The agent then has no description for its next observation choice. The agent then has no description for a skill install.

Drop one function per principal. One function then serves overseer and agent alike. The overseer then sees too little to judge or too much to stay safe. A single function also leaves a principal with no function of its own.

Drop the precedence rule. Order of arrival then decides conflicts. The agent itself then decides conflicts, and the decider becomes a principal. A conflict with no rule then proceeds without suspension.

Drop progress over histories. Judgement then turns step-local. Every step then looks safe while the run never terminates. A run then loops forever with no forbidden step. Vague goals in G then give no test for progress.

Drop the rule that governs design decisions. The party a rule constrains then rewrites that rule. An uncovered case then gets an improvised decision, not a frozen value.

## Membership: what counts as a harness

A candidate is a harness exactly when four things hold.

1. Loop. One loop serves one policy on one task. The next step depends on the previous observation.
2. Designer-controlled machinery. Designer-controlled machinery sits outside the policy write set. A permitted write scope forms the only exception.
3. Action. At least one action changes w or h. A read alone never counts as action. A reshape of the machinery for the next observation counts as action.
4. Designer-fixed control. A designer-fixed control alters or suspends execution without policy cooperation.

Weakness differs from absence. A loop that appends all history to context, with no curation, is a bad harness, not a non-harness. A minimal loop counts as a minimal harness when it runs the test suite and marks a task complete only when the suite passes. Its check does not depend on the model word.

The neighbours fail the test as follows.

- A framework fails the loop test: it composes agents but runs no loops.
- An SDK fails the loop test: it offers loop primitives but runs no loop.
- A plugin fails the loop test: it suggests text with no task state and no verification.
- An orchestrator fails the loop test: it chains steps with no branch on observation.
- An evaluation harness fails the loop test: it grades after the run with no inner loop for a single task.
- A runner fails the test: it launches a harness but runs no loop of its own.
- A bare prompt fails the loop test: it runs no loop.

Scaffold, workflow, application, and agent need the same test. A scaffold passes the test when it runs the loop under designer-fixed controls. Starter code or unfinished developer code fails the test: it offers parts of a loop and runs none.

A workflow with no observation change to its course fails the loop test. A workflow that retries, replans, or branches on observation has a loop for that stretch. That stretch then passes or fails as a loop does.

An application and an agent as a product are bundles, not candidates. Apply the test to the harness part of the bundle, not to the bundle as a whole. A policy alone fails the test. A bundle with no loop, no designer-controlled machinery, or no designer-fixed control fails the test, however complete it looks as a product.

Two corrections follow from the core. A test that counts only actions that change w misclassifies RLM. RLM REPL writes, notes, and skill installs change no world state. Those writes change the next observation. Their state component changed is h, not w.

A control counts by author, not by form. A rewritable cap counts as a suggestion, not a control. A cooperative guardrail counts as a suggestion, not a control.

For any candidate, name the policy, loop, state component changed, and designer-fixed control outside the policy write set. If any item lacks a name, the candidate is a neighbour, not a harness. Membership is binary. Goodness is comparative.

## A worked example: SWE-agent

SWE-agent takes a GitHub issue and a repository at the buggy commit. Its task is a patch that fixes the issue (Yang et al., 2024, arXiv:2405.15793).

DefaultAgent.run names the loop. The file sweagent/agent/agents.py holds DefaultAgent.run. The loop calls step in each pass and stops when step_output.done turns true. Each step queries the model, parses thought and action, executes the action in a container, and saves the trajectory. The trajectory is the saved record of steps in the run.

Templates, tool bundles, history processors, cost limits, blocklist, and timeouts form configuration. The file config/default.yaml and the agent and model configs hold that configuration. The designer sets that configuration, not the model.

The policy acts on both w and h, though unevenly. World-directed moves edit files, write a reproduction script, and run the test suite. Harness-directed moves write the trajectory through the harness at each step. The harness builds the next observation from that trajectory.

Designer-fixed controls alter or suspend execution without policy cooperation. Per-instance and total cost limits terminate the run when the run exceeds either limit. A command times out after 30 seconds. Total execution times out after 1800 seconds. Repeated consecutive timeouts terminate the run.

A blocklist refuses interactive and escape commands. The harness asks again, up to three times, for a different action after a block.

A move identity is its parsed intent and its state component changed, not its surface form. The moves are bash plus configured bundles: a registry, an editor, and a review-on-submit bundle. Because bash is a move, A(h) is not a closed list. Any shell string counts as a candidate move, minus the blocklist, minus budget and time left.

Observation is a function over the whole trajectory, not only over the container current state. The default history processor is the identity. It leaves the trajectory unchanged. The classic LastNObservations processor keeps five observations.

It replaces older observations with the text "Old environment output: (n lines omitted)". A designer-fixed processor thus rewrites history between steps, with no agent action. This rewrite works only because h includes history and trajectory lists, not just container state.

Termination and verdict stay separate. The loop terminates on submit, forfeit, cost, context, or timeout. Autosubmission after an error submits the current diff. A diff is the patch as it stands at submit time. A run therefore terminates as submitted without success.

Success judgement sits outside the loop. The SWE-bench evaluation hook runs the maintainer test suite afterwards and grades the patch. That hook acts as a separate evaluation harness. Inside the loop, the review-on-submit bundle checks the model diff on submit.

SWE-agent has no live observation function for a second principal. An overseer receives the trajectory file and inspector after the run, not an observation function of its own during the run. A human model replaces the policy. It never intervenes in a run already under way.

SWE-agent passes the membership test. A pass on the test does not prove goodness. Its open shell leaves its control reactive. The control reacts to moves. It sets no advance bound on moves.
## Human intervention in a run

Basic question 10 asks how a human pauses, edits, redirects, or takes over a run already under way. The four verbs differ in what they change and in when the change takes effect.

SWE-agent gives no live intervention. It runs as a batch loop. Policy replacement in SWE-agent starts a different run with the same loop. That replacement is not intervention in a run.

OpenCode gives the opposite case. The code inspected here comes from anomalyco/opencode, dev branch, commit 17b79f4e0c16dcff200b0417567ed22aeb2d7166.

### Pause

Session.interrupt stops an active session. The harness records the reason as "user". The interrupt suspends transitions and does not terminate the run. The harness acknowledges the interrupt as soon as it accepts the interrupt. Cleanup settles after acceptance, on a separate path. An interrupt to an idle session changes nothing.

### Redirect during a run

A new prompt does not rewrite the step in flight. The prompt enters the session inbox as steered input for a step boundary during work. The prompt enters the session inbox as queued input for the next turn.

An interrupt with resume wakes the steered input. Queued prompts stay parked after that interrupt. No human edit touches a step already in execution.

### Edit

A revert returns a session to an earlier message. The revert restores files from snapshots. The revert stays staged and gated on idleness. Stage, clear, and commit refuse with a busy error during active execution. The order is fixed: pause first, edit second, and resume third. An interrupt alone rewinds no landed world effect and restores no file.

### Replace the policy

A switch of agent or model publishes a selection event. The session, its history, and its harness state persist through the switch. The policy changes and the run continues. That change is takeover in the narrow sense. It differs from SWE-agent replacement, where a new run starts. It differs from a user who types the next step.

Two interruptions need separate treatment. An interrupted provider stream recovers inside the step. The harness sends a synthetic observation that tells the policy to continue without repeat of completed content. A user interruption aborts the step. The harness fails the step as "Step interrupted". The harness fails unsettled tools as aborted and sends no continue instruction.

The next drain starts a new step from the history. Completed tool results survive in the history. Aborted calls stand as failed in the history.

The theory assumes the user holds write permission for interrupt, steer, and revert. The harness records the reason for each intervention. The harness shows that reason to each party affected by the intervention. Without that record, an aborted step looks like a policy failure. Without that record, later parties lack a basis to judge the step.

### Settlement

A session counts as active through interruption cleanup and terminal settlement. A second interrupt in that window changes nothing and returns false. The second interrupt still claims each recorded resume request. Settlement therefore starts no next step for a dead intent. A resume request that arrives during cleanup counts as a new admission. That request starts a new step when the run settles.

A caller that needs settlement, not acceptance, uses awaitIdle. awaitIdle waits through each new step too. A new step scoped to steered input never promotes queued prompts.

### Partial steps

Resumption after a user interruption restarts the step. Resumption does not continue the interrupted step. The interrupted step fails, and its unsettled tools fail as aborted. Its completed tool results stand in the history. The next drain runs a new step with its step count reset. The new step starts only once a promoted input gives it work to do.

### Precedence

The code gives no rule that ranks principals. Each mechanism decides precedence for its own case. No overseer party ranks principals across mechanisms.

- An interrupt beats in-flight work without condition.
- An interrupt does not control work that arrives after it.
- A resume request during cleanup starts a new step at settle.
- Between two interrupt reasons, no rank order exists.
- The first interrupt stops execution.
- A second interrupt returns false.
- Arrival order decides that pair.
- An active execution beats an edit.
- At promotion, steered input beats queued input.
- A permission rule blocks a single move with no request to a principal.
- A user reply beats all open permission requests.
- A user reject fails the whole batch of requests.
- An interrupt cancels an open permission wait.

The batch case stays open. A harness with no live observation function and no interrupt endpoint lacks live intervention. That harness leaves only a new run as intervention. Whether that act counts as intervention or only as replacement stays open.

## Error model: what fails and who decides next

Malformed output, refusal, loops, and tool failure or timeout need separate treatment. Error is not one kind of event. Failures differ in whether the harness attempted a move. Failures differ in who refused or failed the move. Failures differ in whether the run continues, retries, or terminates.

### Parse failure

A token sequence that the parser fails to map to a move is not an action. The harness still responds to that sequence. The response is an error observation, a retry, or a halt. A transition rule in the error model decides that response. The policy does not decide that response. The action space does not decide that response.

### Refusal before attempt

A move refused before attempt is not a world failure. A blocklist or an approval mode that never asks disables the transition by designer-fixed control. The control reports a disabled transition, not a world refusal. SWE-agent shows the shape. SWE-agent executes no blocked command. SWE-agent asks the policy again, up to three times, for a different action.

### World failure

The world fails an attempted move and the harness gives an observation about w. That observation is not a harness defect. That observation is not a control verdict. An observation that hides the failure source leaves the policy blind on retry. The policy lacks a basis to judge whether a retry of the same move is sensible or futile.

### Refusal and loop

A model refusal, emitted as text, is a parsed move. That refusal is not a harness failure. The policy chose to say no. The harness question is what the harness does with that answer. A loop passes each step check and fails only over the trace. Counters and timers over histories judge the loop.

### Harness failure

The harness itself fails in two opposite cases. An interrupted provider stream in OpenCode recovers inside the step. The harness sends a synthetic observation that tells the policy to continue from the point of interruption. That repair fixes the observation pipeline, not a policy error. A user interruption gives the other case. The step fails as aborted, unsettled tools fail as aborted, and completed tool results survive in history.

### Resource exhaustion

Resource exhaustion is a stop rule, not a policy error. Cost, context, and timeout limits terminate a run by designer-fixed control. A single command timeout differs from a timeout that terminates the run. The first timeout gives a failed move that the policy learns from. The second timeout fires the stop rule for the loop.

The theory assumes the harness marks the failure kind in the observation. Without that mark, the transition rule lacks a basis to choose error, retry, or halt.

### Open points in the error model

- The reason for three requeries stays unexplained.
- What happens at the retry bound stays unjudged.
- Timeout classification stays unsettled.
- A slow tool counts as a world failure or as a harness clock.
- The observation needs to tell those two cases apart.
- Whether provider errors and tool errors need different trajectory marks stays open.
- Whether history processors preserve that mark stays unexamined.
- No general rule covers refusal.
- A policy refusal of its task differs from a tool refusal of a command.

## Cost, budget cap, latency, and access control

Basic question 9 asks what cost, latency, budget cap, and access control constrain, and where each constraint lives. The four are not one thing. A veto misread as a price, a stop rule misread as a policy error, or a permission misread as a budget cap misattributes the constraint.

### Cost

Cost lives in the trace and in h. A principal observes cost through an observation function. The Codex CLI adapter gives the overseer cost records. SWE-agent gives its overseer only the trajectory file after the run. Cost spans tokens, tool calls, and world-directed effects.

The harness counts tokens after generation. A tool call carries an execution cost unknown at choice time. A world-directed effect, such as a message sent, escapes any token count. No move therefore carries a single price that the policy weighs beforehand.

### Budget cap

A budget cap is not a price. A budget cap is a designer-fixed cap in the core. Exceedance of the cap terminates the run. SWE-agent carries a per-instance cost limit and a total cost limit. A budget cap shapes A(h) from the outside. A budget cap is not a move.

A budget cap is not a price that the policy trades against a larger payoff. A cap outside policy write permission binds the policy. A cap that the policy rewrites is a suggestion, not a cap. An agent-raised budget cap is evasion, not compliance. The harness measures cost, and the designer fixes a budget cap before the run.

A budget cap vetoes the run at a limit, whatever the payoff of the next move. A price changes total cost and leaves the run to continue. A blocklist vetoes one move, whatever the budget left. SWE-agent shows the blocklist case: a blocked command runs for no price.

### Latency

Latency runs on two clocks, and the error model keeps the clocks apart. A per-move clock times a single command. A single command in SWE-agent times out after 30 seconds. That timeout fires a failed move that the policy learns from. A per-run clock times total execution. Total execution after 1800 seconds terminates the run.

The first timeout is not a run failure. The second timeout is not a policy error. Latency also enters progress as a timer. The timer measures time since the run last made progress toward G. A slow tool and a stuck loop look alike until the timer crosses its threshold. At that point, delay counts as a verdict.

### Access control

Access control shapes A(h) directly. Access control says which moves the harness attempts. Admission lists the moves that exist. The harness attempts no move outside that list. A veto is a designer-fixed rule that disables an attempted move. SWE-agent blocklist refuses known bad commands.

Codex CLI refuses commands under its forbidden exec-policy rules. Approval means a party decides a request. Codex CLI approval comes from a guardian or a user. A grant lasts for a turn or a session. The harness writes no grant to disk. Under default on-request approval, the policy chooses whether to seek approval.

Sandbox rules and forbidden rules hold regardless of a request. A durable amendment differs from a grant. A grant changes attempts for a turn or a session. An amendment changes the rule itself. An amendment is a recorded design decision. Access control is neither a precedence rule nor a budget cap.

A precedence rule decides between proposals. A permitted move stays unaffordable in some runs. An affordable move stays forbidden in some runs.

### Open points in cost, latency, and access control

- Whose observation function carries cost during a run stays open.
- The harness charges an inner loop cost to the outer move that spawned it.
- The summary hides the inner steps.
- Latency as a progress proxy stays open.
- Which permission changes the harness grants mid-run stays open.

## Kinds verdict: one theory for coding, browsing, chatting, and operating harnesses

Basic question 11 asks whether coding, browsing, chatting, and operating harnesses belong to one theory. This section considers the question and does not resolve it. Every adapter so far is a coding harness. The other kinds stand as kinds, not as verified systems.

The membership test is kind-blind, and that fact favours one theory. The test asks for a loop. The test asks for designer-controlled machinery outside policy write permission. The test asks for an action that lands somewhere. The test asks for a designer-fixed control that alters or suspends execution without policy cooperation. None of the four mentions files, pages, messages, or clicks.

A loop that navigates pages passes the same test that SWE-agent passes. A loop that replies in a conversation passes the same test. A loop that drives a desktop passes the same test. A loop that fails any of the four fails the test in any kind.

Passage of the same test does not make two harnesses the same thing. The core stays put, but three applications change shape by kind.

### The world and harness line and the principal count

In coding, the repository is w, and the trajectory, the templates, and the registry are h. In browsing, the web is w. In browsing, tabs, cookies, and a cached page are h. Cached divergence from the live world is therefore a harness failure. In operating, the desktop and its files are w. In operating, focus, the clipboard, and the screenshot pipeline are h.

Chatting is the sharpest case. A reply lands in the transcript in h. That transcript serves another principal's observation function. The interlocutor is a principal who judges, interrupts, and terminates a run. A harness that treats the interlocutor as world loses observation functions, precedence rules, and principal-judged progress. A harness that treats the interlocutor as a principal makes chatting multi-principal from the start.

Single-policy SWE-agent contrasts with that case. The kind changes the principal count, not the core.

### Termination and verdict by kind

In coding, the loop terminates on submit. Tests outside the loop judge success. In browsing, no submit exists. A browsing loop terminates at a page state. A checker that reads a world that changes states whether it reads w or a harness snapshot.

In operating, system state is both work and verdict. A stop therefore mimics success in operating, unless a check separate from the loop is defined. In chatting, a reply closes a turn, not a run. A chatting run terminates when the interlocutor terminates it or a stop rule fires. Progress in chatting is principal satisfaction, not a change in w. Liveness over histories still applies in chatting.

A principal other than the designer holds G in chatting.

### Irreversibility and the shape of the attempted moves

In coding, open-shell moves are veto-based and often reversible by diff and revert. Review after the act repairs those moves in coding. In browsing, links and buttons on the current page offer candidate moves. A(h) lists moves available to attempt, not guarantees, in browsing. The world offers, the harness attempts, and the world determines success.

The attempted-move set takes h only, not w. An attempt is a decision of the harness. The harness decides from h. h holds the registry, the access control, the skills, the budget cap, and the recorded observations. A world offer reaches the harness as an observation. That observation then enters h as recorded history. The offer changes A(h) through h, not directly. A cached page shows the test. A cached page in h offers a move. The world no longer offers that move. The harness still attempts the move, and the move fails. The attempt follows h. Success follows w. A set that takes w as an argument mixes the two. It makes the attempt set depend on w. The harness does not observe w directly. The harness observes o. A set that the harness cannot compute from h is not an attempt set.

In operating and chatting, moves are often irreversible. A message sent, a file deleted, and a purchase placed show that shape. Control in those kinds is pre-emptive, by admission, veto, or approval before the move. Post-hoc review does not suffice for those kinds. Review-on-submit is therefore inadequate for operating and chatting. Membership requires only some designer-fixed control, not that control shape.

Which control shape is adequate is goodness, not membership.

### The browsing boundary

Browsing strains the boundary in one further way. Page content is w and is untrusted. Page content contains instruction-like text. Control draws the boundary, and a page lacks designer control. A harness that treats page content as a designer instruction misattributes the source. The policy errs or the harness fails when it follows that content.

A browsing harness keeps world content and designer instruction in different channels or observation functions. Without that separation, the boundary collapses in practice.

A kind splits the theory only if it requires a new core claim. A kind splits the theory only if it systematically fails membership. The candidates do neither.

- Whether chatting progress needs its own verdict vocabulary stays open.
- A second, non-coding adoption stays open, and this section stays prospective until that adoption.

## Basic questions

These questions come before the more elaborate parts of the theory. They do not require a resolved answer. What a theory of harnesses owes each of them is consideration: an explicit account of how the question bears on the theory, what the theory takes for granted about it, and where it leaves the question open, even if no settling answer is in sight.

1. What is the action space? Is an action a token, an API response, or a tool call, and how do those levels relate? Can the action space change as the harness state changes?
2. Is a partially observable Markov decision process the right mathematical framework? What does it capture, what does it distort, and what should complement or replace it?
3. What are the major harnesses used today, and what varies among them? Coding agents, frameworks for building agents, and research prototypes are not the same kind of thing.
4. What is a step, a turn, and a run? When does a run end, who decides, and what distinguishes stopping, failing, and being halted?
5. What is state, and how does it differ from context, memory, and the world? What is in `s`, and what is only text shown to the model?
6. What is a tool, as opposed to a subagent, a prompt, a skill, a memory, or a retrieval result? Are these different kinds of things, or the same thing under different names?
7. What is a harness, as opposed to a framework, a scaffold, an agent, an application, or a workflow?
8. What happens when the model produces malformed output, refuses, loops, or calls a tool that fails or times out? What is the harness's error model?
9. What do cost, latency, budgets, and permissions constrain, and where do those constraints live?
10. How can a human intervene in a run already under way: pause it, edit it, redirect it, or take over?
11. Do coding, browsing, chatting, and operating harnesses belong to one theory, or does the word "harness" cover different things in each case?

## Open questions

1. What is the minimal core that all harnesses share, without being so general that it says nothing? POMDP alone is too general to guide design.
2. What distinguishes a good harness from a merely working one, once cost, robustness, legibility to humans, and failure modes are included, not just reaching `G`?
3. Is agent control over observation the central principle, or one principle among several, alongside control over transitions and legibility?
4. How should a theory handle multiple principals at once: the agent, the user, and the human overseer, each needing their own observation function?
5. What invariants survive across good harnesses, across models, and across tasks?
6. How can a formalism be useful now and into the future, without knowing the future? What should it commit to, and what should it leave open, so it can be revised gracefully?
7. What would a harness look like if it were coded from a solid mental model, so cleanly that new developments fit into it without tangling, and maintenance stays low?

## Sources to read, test, and argue with

- Recursive Language Models, Zhang, Kraska, Khattab, arXiv:2512.24601, first posted December 2025.
- Prime Agent, Prime Intellect, August 2026, a self-improving harness on the RLM abstraction. §2.5 gives the fixed-core and mutable-supplement split.
- Classic reinforcement learning and decision theory: Sutton and Barto, and the older MDP / POMDP literature. Ch. 3 gives the designer-control test.
- Supervisory control: Ramadge and Wonham, "Supervisory Control of a Class of Discrete Event Processes", SIAM Journal on Control and Optimization, 1987. It gives the veto-without-compulsion shape for the precedence rule section.
- Safety and liveness: Bowen Alpern and Fred B. Schneider, "Defining Liveness", Information Processing Letters 21(4), October 1985. It gives the safety and liveness decomposition for the liveness section.
- Vague goals and aspiration: Herbert A. Simon, "A Behavioral Model of Rational Choice", Quarterly Journal of Economics, 1955. Satisficing replaces a point goal in the progress section.
- Succession fixed before vacancy, as illustration, not authority: the United States Presidential Succession Act of 1947 and the Twenty-fifth Amendment, and the Linux kernel's succession and continuity plan. It illustrates legitimacy from a rule fixed before a vacancy.
- Software sunset practice, as precedent for announced, drained endings, not as authority: deprecation announced before shutdown, with a migration path and a date (IETF, RFC 8594, the Sunset header). It gives precedent for announced shutdown with a migration path and a date.
- Novation in contract law, as illustration, not authority. It illustrates the fourth inheritance in the successor section.
- Trending and historical harness implementations on GitHub, to be read as examples, not as authorities. A working implementation counts as evidence and does not by itself give a theory.
- Corporate successor liability, as contrast, not authority: the traditional asset-buyer rule the chain section inverts, since a harness successor claims the designer role, not the assets. The chain section inverts the traditional asset-buyer rule.
- Identity across revision, as illustration, not authority: Plutarch, Life of Theseus, on the ship of Theseus, and Hobbes's second ship built from the discarded planks; and H. L. A. Hart, The Concept of Law, 1961, on the rule of recognition. It gives the recognition rules for the identity section.
- Tamper-evident records, as illustration, not authority: hash-chained append-only logs and Merkle trees, as in version control and Certificate Transparency (IETF, RFC 6962). It gives graded trust and a bound on forgery for the trust section.
- Trust roots, as illustration, not authority: a root certificate in public-key infrastructure is installed by the relying party, not derived from a chain. It gives the adopted root for the regress section.
- Trust anchor rotation, as illustration, not authority: IETF, RFC 5011, "Automated Updates of DNS Security (DNSSEC) Trust Anchors", September 2007. It gives the overlap, closing link, and emergency rule for the rotation section.
- Agent harness definition, as foil and test, not authority: Sanderson Oliveira de Macedo, "What makes a harness a harness: necessary and sufficient conditions for an agent harness", arXiv:2606.10106, August 2026. Its four conditions and neighbour boundary serve as foil and test for the membership section. This document departs from it in two places. A harness-state action counts as an action. A control counts only if designer-fixed and outside policy write permission.
- SWE-agent, as worked example, not authority: John Yang et al., "SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering", arXiv:2405.15793, 2024, and the system's current code and default configuration. Its loop, action set, and controls serve as the worked example.
- Planning under partial observability, as the formalism judged, not authority: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, "Planning and acting in partially observable stochastic domains", Artificial Intelligence, 1998. It gives the division of labour for the POMDP section.
- Temporally extended actions, as complement, not authority: Richard S. Sutton, Doina Precup, and Satinder Singh, "Between MDPs and semi-MDPs: a framework for temporal abstraction in reinforcement learning", Artificial Intelligence, 1999. It gives the treatment of a subagent move as a semi-Markov action at the outer level.
