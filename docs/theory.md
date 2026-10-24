# Theory

This document states the first definitions. It also states the questions that the definitions leave open.

## Starting definitions

w is the world state. w holds the state of the world outside the harness. h is the harness state. h holds the state inside the harness. s is the full state.

s is the pair (w, h).

o is an observation. o is what the agent sees at one step. In the case of a large language model, o is the full context history. a is an action. At the lowest level, a is one token or one log-probability.

The network generates a.

pi is a policy. A policy is a rule for action choice. pi(a | o) gives the action choice for observation o. An agent is pi(a | o). s' is the state after one step from s.

p is a probability function.

p(s' | s, a) is the transition. A transition is a change from s to s' after action a. p(o | s) is the observation function. An observation function gives o for state s. The harness is part of the environment.

The harness carries out p(s' | s, a) and p(o | s).

A harness is a system that the designer builds around the agent. The designer is the person who builds the harness. The harness carries out the transition and the observation function. A tool is an engineered shortcut. The harness jumps to a state that holds the answer.

The harness then shows the answer to the agent. A dummy case is a simple example. The weather today is a dummy case.

## The harness boundary: what the designer controls

A boundary is a split by control. Sutton and Barto draw the boundary by control. Environment is what the agent does not change at will. The same rule draws the harness boundary. The rule splits the environment three ways by who changes what.

The agent is what the agent controls. The agent controls its policy. For a large language model, the agent controls its weights. The agent does not control its prompt. The harness is what the designer controls.

The designer controls the harness with no change to the agent. The designer controls the harness with no change to the world. The designer controls the transition and observation machinery. The designer controls internal state, interfaces, budgets, and checks. The world is what neither the agent nor the designer controls.

The world holds the weather. The world holds answers from external services. The world holds retrieved documents.

Apply this test to each part of the system. The designer changes the part with no change to the agent. The designer changes the part with no change to the world. Retrain of the agent puts the part on the agent side. A change to the world puts the part on the world side.

No other result leaves the part in the harness.

Do not define the harness as all that lies outside the agent. That definition folds the lever of the designer into the world. Attribution is the assignment of a failure to its cause. That fold makes failure unattributable. The system prompt is harness, not agent.

The designer rewrites the system prompt with no change to the weights.

## The boundary, one level up: self-modifying harnesses

A self-modifying harness is a harness that rewrites itself. The Continual Harness from Prime Agent is a concrete case. Prime Agent is a system with a self-rewriting harness. A self-rewriting harness looks like a counterexample. It looks like a counterexample because designer and agent cease to be separate parties.

It is not a counterexample at the right level. Apply the split at the right level.

Split the harness into a fixed core and a mutable supplement. The core is fixed. The supplement is mutable. The core holds the immutable base prompt. A turn boundary is the end of one turn.

The core holds the runtime that applies edits at turn boundaries. Provenance is the record of the origin of each change. Rollback is the return to an earlier version. The core holds versioning. Versioning records provenance and enables rollback.

A subagent specification is a description of a subagent. The supplement holds prompt notes, memories, skills, and subagent specifications. A skill is a stored ability. The harness adds a skill to the harness state.

The edits of the agent are ordinary actions. A modification policy is a rule for change. The edits stay within a modification policy that the designer fixes. The rule allows edits to certain parts. The rule allows edits at certain times.

The rule allows edits on certain evidence. The rule requires provenance for each edit.

The test survives one level up. The designer controls the core and the policy. The designer does not control the harness in operation. Attribution survives also. A bad skill that the agent wrote is agent-side.

A bad core is designer-side. A permissive policy is designer-side. A failed rollback is designer-side. A write set is the set of parts that the agent has permission to write. Attribution holds exactly in one case.

In that case, the modification machinery lies outside the write set of the agent. Prime Agent keeps its runtime outside the writable state. This document does not cover a harness that learns its own modification policy from experience.

## The action space: a tower, not a set

A tower is a stack of levels of description. A level is one layer of description in the tower. A POMDP is a model with a hidden state and observations. Each level has its own action set. Each level has the same POMDP shape.

A is an action set. A holds the actions at one level of description. No single set A serves every level of description. A run is a sequence of steps at one level. Expansion is the unfold of one action into a run at the level below.

Expansion relates the levels to each other. An executed action unfolds into a run at the level below.

V is the vocabulary of the model. At the lowest level, the action set is V. At the lowest level, the agent takes one token per step. pi(a | o) is a distribution over V. A distribution gives a probability for each choice.

At the lowest level, nothing else is an action.

A1 is the next level of action. At the next level, the harness groups token sequences into parsed moves. A parsed move is a move from tokens. The harness parses the tokens. A1 holds four kinds of parsed move.

- The agent emits text.
- The agent calls a tool.
- The agent spawns a subagent.
- The agent stops.

A1 is a quotient, not a subset, of token sequences. A quotient groups distinct spellings into one item. A parser is a part that reads tokens into moves. A registry is a list of tools and moves that the harness knows. Two spellings of one tool call are one action.

Under different parsers, the same text is a different move. Under different registries, the same text is a different move. A move has an identity. The identity of a move is its parsed intent plus its target in s. Parsed intent is the intent that the parser reads from the tokens.

The target in s is the place in s where the move acts.

A move executes into w, into h, or into both w and h.

A loop is a sequence of steps that repeats. An inner loop is a loop inside a move. A tool move takes one harness step. A subagent move runs a whole inner loop. The subagent move returns only a summary.

The inner loop is itself a harness.

A(h) is the set of attempted moves in harness state h. Write A(h), not A. State-dependent means that the set changes with h. The action set is state-dependent. Registry, permissions, skills, and budget are all harness state.

Registry, permissions, skills, and budget add moves. Registry, permissions, skills, and budget remove moves. Install of a skill gains moves. Loss of a tool loses moves. Exhaustion of budget leaves no costly move.

A(h) lists attempts, not guarantees. Success still depends on w.

Tools and subagents are moves. Tools and subagents differ only in execution. A tool move executes in one step. A subagent move executes in one loop. Skills, prompts, and memories are not moves.

Skills, prompts, and memories are harness state that shapes the tower. A skill changes moves and the parser. A prompt shapes the next observation. A note changes a later observation. Install of a skill changes A(h) later.

A retrieval result is an observation, not an action.

A principal is a party with its own view. A view is what one principal sees. A chat reply lands in the transcript in h. The chat reply is for the view of another principal. Target and audience differ for a chat reply.

Unparseable token sequences are not failed actions. Unparseable token sequences are no action at all. The harness still responds to an unparseable token sequence. The harness responds with an error, with a retry, or with a halt. A transition rule in the error model of the harness governs the response.

An error model is a set of rules for errors. The action space of the harness does not govern the response.

Invert the cut for a last point. In the inverted cut, the harness code is the policy. In the inverted cut, the model is environment. The tower stays the same in the inverted cut. The cut differs in the inverted cut.

## Is a POMDP the right framework?

A POMDP is a model with a hidden state and observations. A tuple is one fixed group of parts. A reward is a number that judges one step. A POMDP is one tuple. The tuple holds one state set, one action set, and one observation set.

The tuple holds one transition, one observation function, and one reward. For one step, a POMDP is the right framework. For a harness, a POMDP is not the right framework. A POMDP is the shape of a step. A POMDP is not the theory of the steps.

The agent never sees w directly. The designer designs much of what the agent sees in h.

First, a POMDP has not one action set for a harness. The tower is a family of POMDPs. The family has one POMDP per level. Parser choice and expansion relate the POMDPs in the family. A rejected token sequence is in neither action set.

A subagent move is a temporally extended action. A semi-Markov process is a process with actions that take many steps. An option is a temporally extended action in a semi-Markov process. A subagent move is an option, not a one-step move.

Second, a POMDP has not one observation function for a harness. Each principal needs its own observation function over the same s. Each observation function gives o for state s for one principal.

An epoch is a period with fixed harness functions. A design act is an act of design after the start of a run. Third, the functions of a POMDP stay fixed. The functions of a harness are not fixed, except within an epoch. Later design acts change registry, parser, and thresholds.

A veto is a refusal, not a cost. A price is a cost. A blocklist is a veto, not a price. Aspiration is a target that moves. An aspiration is what the harness aims at over time.

G is a fixed goal point. Aspiration and epochs replace G with a target that moves and its revision record. A revision record is the record of each revision of the target. Within an epoch, a run is a POMDP. Across epochs, a run is a family indexed by design acts.

Design acts index the family across epochs.

Evidence is information about w. A designed artefact is a thing that the designer builds. Fourth, the observations of a POMDP are evidence. The designer builds the observations of a harness as artefacts. SWE-agent is a harness.

A history processor is a part that rewrites history. An omission note is a note that states an omission. A trajectory is the record of steps in a run. The history processor of SWE-agent replaces older observations with an omission note. The harness builds the next observation from the rewritten trajectory.

The case fits only because trajectory lists are in h.

A belief is a judgement about what is true. A belief from a designed record is a belief about the record. The belief is about what the designer chose to show. The belief is not about w.

A trace is the full record of one run. Fifth, the judgement of a POMDP is a step reward. The judgement of a harness is partly over traces. Liveness is a whole-history property. A whole-history property depends on the full history, not on one step.

Liveness holds four parts.

- Deadlock is a state where no move proceeds.
- Starvation is a state where one move never proceeds.
- Livelock is motion with no progress.
- Progress against aspiration is advance toward the target that moves.

No step reward encodes liveness. A step reward that encodes liveness smuggles in the trace judgement. Smuggle in means hide inside without a clear statement.

Vacuity is emptiness of meaning. Markov means that the present state holds all past information for the next step. Finally, consider the vacuity. If s and o are each the whole history, any process is Markov. If any process is Markov, the claim "a run is a POMDP" is true and empty.

The claim is informative only for a fixed s. For a fixed s, ask these questions.

- Which POMDP holds for this run?
- Which level holds for this run?
- Which epoch holds for this run?
- Which principal holds for this run?
- Which vetoes hold for this run?

Keep the POMDP as local shape, one level, one epoch. The rest goes around the POMDP. A designed thing is a thing that a designer builds. A formalism is a formal system for description. A formalism that does not state who changes its own transition function does not serve as a theory of a designed thing.

## Goals

A harness is the system around an agent. The harness shapes the work of the agent. The designer fixes the agent. The designer varies the harness. The variation makes it easier to reach a set of goals. G is the set of goals for the run.

Specification of G is hard in a wide form, especially where people take part. Some goals are not about agent performance at all. A human can monitor the health of the system with ease.

G shifts over time. Designers generally make harnesses easy to modify. A theory that cannot survive a change in G is not yet a theory.

## An example that clarifies the framing

A Recursive Language Model (RLM) is a language model that works recursively. A REPL is an interactive tool for code. In an RLM, a long prompt lives as a variable in a Python REPL.

The root model sees only metadata about the prompt. The root model writes code to inspect the prompt. The code decomposes the prompt. The root model calls sub-models on pieces of the prompt. Each call works on a piece of the prompt.

The power is not Python as such. The agent gains control over its own next observation. An observation is an item of information that the agent receives.

Observation is endogenous in an RLM. Endogenous means that the actions of the agent produce the next observation. Observation is adaptive in an RLM. Adaptive means that the next observation changes with what the agent learns.

The agent explores. The agent acts on what it learns. The agent explores again.

An agent that chooses what to see faces three risks:

- The agent can cherry-pick evidence.
- The agent can miss evidence against a claim.
- The agent can stay stuck in a plausible branch.

Control over observation moves harness design up a level. What observation actions are available? What do they cost? What forces a check?

## One principle or several? World-directed and harness-directed actions

A state is the full situation at one step. The state has the form s = (w, h). World state (w) is the part of the state that neither the designer nor the agent controls. Harness state (h) is the part of the state that the designer controls.

An action is a step that changes the state. A world-directed action changes w. A world-directed action does one of these acts:

- send a message
- place an order

A harness-directed action changes h. A harness-directed action writes one of these items:

- a REPL variable
- a memory
- a skill
- a plan

A transition is a change from one state to a next state.

o is an observation. p(o | s) is the observation function. The function gives the probability of observation o in state s.

Control over observation is the harness-directed case from the observation side. A change to h changes what p(o | s) delivers next.

Control over transitions is the harness-directed case from the transition side. A change to h changes which transitions are available next.

The principle is this. A harness gives the agent partial authorship over harness state. That authorship gives the agent control over its own future observations. That authorship also gives the agent control over its own future transitions.

In an RLM, the agent writes a REPL variable. That action shapes the next observation.

The job of the harness then shifts up a level. The harness no longer only delivers observations. The harness governs authorship.

Which harness state may the agent write? At what cost? With what checks? With what provenance?

The split is relative to the boundary that the designer draws. The split is not absolute.

A view is what one party sees of s. Legibility to other principals is separate. Legibility is about views on s. Legibility is not about the actions of the agent on s.

## Multiple principals, one state

A principal is a party with three items:

- its own observation function
- its own write set
- its own goal

An observation function maps a state to an observation for one principal.

The agent observes the context history. The agent writes harness state within the permitted scope. The agent pursues G.

The user observes an answer or a summary. The user writes instructions. The instructions set G. The instructions revise G. A revision changes the set of goals. The user judges whether the answer serves the user.

A trace is the history of a run. A policy is a rule that the designer fixes in advance. A modification policy is a policy for changes to the harness.

The overseer observes the trace, costs, checks, and health of the system. The overseer writes the fixed core and the modification policy. The overseer can halt a run. The overseer can roll back a change. The overseer pursues a goal that is not the task at all.

What multiplies is the observation. A single-agent account uses one p(o | s). A multi-principal view uses a family of functions. The family has one function per principal.

Let i name one principal. p_i is the observation function for principal i. o_i is the observation of principal i. For each principal i, the function has the form p_i(o_i | s).

In the weather case, one tool call in s yields three observations:

- The agent gets a structured result for action.
- The user gets "sunny, 21°".
- The overseer gets latency, cost, and a note on success of the call.

A view must satisfy two conditions. The two conditions pull in different directions.

Sufficiency means that the view gives the principal enough to do the job.

Separation means that sight of an item does not give the right to write the item.

When the agent writes h, that action changes what every principal sees next. The designer must decide in advance which writes of the agent each principal sees.

The modification policy is therefore also a visibility policy. The modification policy is not only a write policy.

## Conflict between principals: arbitration is harness, not a principal

Arbitration is the harness rule for a conflict between principals. When goals of principals disagree, the harness decides. The harness decides by a rule that the designer fixes in advance. Arbitration is harness, not another principal. An arbitrator that has its own goal is one more principal.

The core is the fixed part of the harness. A rule in the core makes a resolution binding. That rule decides which transition actually occurs.

Principals propose actions at a step. The arbitration rule of the core selects proposed transitions. The arbitration rule enables the selected transitions. The arbitration rule disables the rest for that step.

Precedence is a partial order on pairs of actions. Precedence is not a total order on principals.

The user revises G. That revision outranks the old goal that the agent continues. The user cannot override a check in the fixed core. That check outranks the urgency of the user.

Arbitration, in this form, is veto, not compulsion. Veto means that the harness disables a transition. Compulsion means that the harness forces a transition. The plant is the system under control. The supervisor of Ramadge and Wonham disables a subset of controllable events. That supervisor has no means to force the plant to generate an event.

Preemption means that precedence disables a proposed transition. The view of the preempted principal must show the preemption. The view must also show the rule behind the preemption.

Two proposed actions can conflict. No fixed precedence decides between the two actions.

In that case, the harness does not invent a precedence at runtime. The harness suspends only the contested transition. The harness lets unrelated work proceed. The harness surfaces the conflict to the designer through the overseer.

## Liveness under precedence: safety is step-local, liveness is history

An overseer that always halts starves the goal of the user forever. A check that always gates also starves the goal of the user forever. Neither case violates a precedence rule.

Alpern and Schneider present the decomposition in "Defining Liveness" (1985). Every trace property is the conjunction of a safety property and a liveness property.

Safety is step-local. A safety property states that no bad outcome happens. A finite prefix already shows a violation of a safety property.

Liveness is history. Liveness is a property of a trace. A liveness property states that a good outcome eventually happens. Every finite prefix still admits an extension that satisfies the liveness property.

Deadlock is a case in which the core enables no transition. No work proceeds in deadlock.

Starvation is a case in which the core always disables transitions of one principal. Transitions of other principals proceed in that case. The run looks alive in that case. One party receives no service in that case.

Livelock is a case in which transitions fire. The run never approaches G in that case.

The classical precedent is the nonblocking supervisor of Ramadge and Wonham. Under that supervisor, every reachable state has a path to a marked state.

The core judges liveness over traces at runtime. The core does not judge liveness over the state graph in advance.

Runtime detection is therefore a rule of the core over histories. The rule uses counters and timers.

A counter counts consecutive preemptions of one transition. A timer measures time since the run last made progress toward G.

When a counter crosses a threshold, the status of the run changes from "held" to "defective". When a timer crosses a threshold, the status of the run also changes from "held" to "defective".

A criterion is a standard for a verdict. The designer fixes the criterion in the core. The criterion states what counts as progress. The criterion states what threshold marks a defect.

A verdict is a judgement on a trace. The core computes the verdict from the trace.

The overseer acts on the verdict. The write set of the overseer already contains the actions for the verdict:

- Abort the run.
- Escalate to the designer.
- Relax the check.

In some cases, the overseer preempts a transition. The view of the user must still show the verdict.

A liveness verdict is not a transition proposal. A liveness verdict does not enter arbitration. A liveness verdict is a status change on the run.

## Progress toward a vague or shifting G: epochs, aspiration, and proxies

A harness is the system that runs an agent and checks the run. An agent is the actor that acts toward G. G is the goal. G is often vague. G also shifts over time.

This section treats G as an aspiration, not as a single point goal. An aspiration is a level that a principal judges as good enough. Herbert Simon calls this idea satisficing (Simon, 1955). A principal holds an aspiration level. The principal can judge the criteria for that level. Search stops at the first state that clears the level.

Search does not seek an optimum. A view is what a principal sees of a run. Progress is not distance to a point. Progress is approach to the current aspiration. The principal judges that approach in the view of the principal.

A proxy is a substitute measure for a goal. A proxy may serve where even aspiration is hard to state. A proxy can take one of these forms:

- new information about the world
- less uncertainty about the world
- less uncertainty about what G itself is

A proxy is legitimate only with a tie back to the aspiration. A principal must use the gain to judge G. A principal must also use the gain to advance toward G.

The write set is the group of principals that can revise G. Each revision comes from a principal in the write set for G. Typically, the user makes the revision. Each item in a goal sequence is one version of G. Write the goal as a sequence: `G_0, G_1, ...`

An epoch is one period under one version of G. Each revision starts a new epoch. Counters and timers run within an epoch. Counters and timers reset only at a recorded epoch boundary. The harness records the provenance of each boundary. Provenance is the record of who made a change and when the change occurred.

The core is the fixed rule set of the harness. The progress predicate is the rule that judges progress. The progress predicate is part of the core.

Masking is a revision that redefines G to a state already reached. That revision does not count as progress within the old epoch. If the principal that revises G accepts the current state under the new goal, the run completes under the new epoch. The harness records that completion as a revision, not as progress.

A silent redefinition that merely resets the timers without a run termination is an evasion, not an epoch. A run termination is the stop of a run. Churn is repeated revision of G. Each revision resets the timers. No epoch ever turns defective. No epoch ever completes.

Count revisions. Count time without a completed epoch. When revisions cross a designer-fixed threshold, the run turns defective. A threshold is a fixed value at which a counter or timer changes a run state. The designer is the authority that sets the core rules.

## Thresholds without a designer: meta-rules, frozen values, and succession

Liveness is the requirement that a run continues to progress. A threshold is a fixed value at which a counter or timer changes a run state. The designer fixes that threshold. At that threshold, a counter or timer changes a run from held to defective. The liveness and progress sections assume that threshold. Those sections do not defend that threshold.

The thresholds already in the core persist as inheritance. Those thresholds stay in force until a legitimate revision changes them. The agent must not set its own thresholds. An overseer is a judge of runs. The overseer must not set thresholds at runtime in the capacity of judge. The harness must not learn thresholds from its own traces.

A trace is the record of a run. A threshold states what counts as a defect. A threshold does not state what usually happens.

A meta-rule is a rule about how to compute a threshold value. The designer fixes a meta-rule in the core. The meta-rule fixes three items:

- how the harness computes a threshold value
- what inputs the harness uses for the computation
- what bounds limit the value

The meta-rule carries one sharp condition. The quantity that the harness judges must not set its own bar. That quantity must not scale with the trouble itself.

Where no meta-rule covers the case, the default is freeze, not relax. The last designer-set values stay in force. Revision remains possible. Revision counts only as a design act. A design act is a recorded change to the core by the designer.

The designer records each threshold change as a designer act. The designer records the provenance of each change. Every principal can see each change.

A verdict is a judgment about a trace. Prospective means that a change applies only to future traces. A threshold change is prospective only. A threshold change does not erase the prior verdict. A threshold change does not retroactively clear the history that produced the verdict.

Legibility means that a view shows a fact clearly. The absence of the designer must be legible in the views. The views show the thresholds as inherited and frozen. The views show that the designer is not currently reachable. The next section treats succession.

## Succession: who inherits the designer role, and on what authority

The designer role is the authority to revise the core. The designer role covers five items:

- the precedence rules
- the progress predicate
- the thresholds and their meta-rules
- the modification policy
- the views

Succession is the transfer of meta-authority. Meta-authority is the authority to revise the core. Succession is not a transition proposal. A transition proposal is a proposal to change a run state. Succession is not a view. Succession is not a verdict.

Arbitration is the process that gates proposals. Arbitration gates proposals. Arbitration therefore does not decide succession. A liveness verdict judges traces. A liveness verdict therefore does not decide succession.

The succession rule must already be in the core before the absence that it governs. A trigger is the event that starts succession. The first legitimate form is a named successor. A named successor is a person, a role, or an ordered list that the core names. Authority passes down the list when the trigger occurs.

The second legitimate form is a fixed procedure. The core names no person under a fixed procedure. The procedure fixes four items:

- who initiates the choice
- who meets for the choice
- within what time the choice occurs
- by what vote or confirmation the choice occurs

The successor that the procedure chooses inherits the role.

Self-appointment is evasion. Self-appointment is a claim to the role without a prior rule. A party that declares itself designer has no prior rule behind it. That party has only its own say-so.

The agent cannot succeed the designer. That succession collapses the agent-designer separation. The whole theory rests on that separation. That succession puts the judged party in charge of the bar.

The overseer cannot promote itself from judge to designer in order to escape a verdict. The overseer cannot set its own thresholds. A promotion by the overseer lets a judge judge its own promotion.

A user majority is more than half of the users at runtime. A user majority at runtime cannot confer the role. A core procedure can confer the role through a vote. Then the procedure confers the role, not the majority.

A legitimate successor inherits the office, not just its powers. That inheritance covers three items together:

- the core as it stands, with its meta-rules and frozen values
- the records, provenance and prior verdicts
- the constraints on design acts

The successor cannot erase the records, provenance and prior verdicts. The successor cannot retroactively clear the records, provenance and prior verdicts. Design acts stay recorded. Design acts stay prospective. Every principal can see design acts.

A delegate acts within a mandate that the designer still holds. The designer can revoke that mandate. A successor replaces the designer. After succession, the old designer cannot revoke the role.

Succession also needs a trigger fixed in the core. Without a fixed trigger, a transfer counts as usurpation, even with a name attached. Once the trigger occurs, the transfer must be legible. The sufficiency condition requires that the views show three facts:

- that the designer is absent
- that the trigger occurred
- that this party now holds the role, under the prior rule

A vacancy is an empty designer role. If no successor exists, the frozen inheritance becomes terminal. Each run completes or suspends under the frozen core. The harness surfaces the vacancy. The harness does not fill the vacancy with whoever acts first.

## Ending: when a harness may properly end, and how it ends well

A harness is the standing machinery that runs work for principals. A principal is a party to whom the harness owes work or a record. A designer is the role that makes design acts for the harness. A design act is an act that sets or changes a rule of the harness.

An aspiration is the goal that an epoch seeks. An epoch is one bounded span of work under one aspiration. A run is one execution of work by the harness. Termination is the close of one run. Ending is the retirement of the harness itself.

Ending differs from the close of a run. Ending retires the standing machinery itself. After ending, the harness accepts no new run. The harness brings each run in flight to a close. The harness discharges each responsibility or hands it on.

Ending is a design act. Ending is not a verdict on past work. The designer role decides an ending.

Ending carries the constraints of every design act. The designer records the act. Prospective means that the act applies only to future work. Every principal can see the act.

A harness that simply stops does not end. No record shows who decided the stop. No record shows why the stop occurred. The principals treat that stop as abandonment.

Only the designer may end the harness. A legitimate successor may also end the harness. Succession is the transfer of the designer role to a successor. The successor acts under the succession rule.

Three legitimate endings exist. Each ending differs in shape. The three endings are not three names for one stop.

- Completion of purpose ends the harness.
- Supersession ends the harness.
- Sunset ends the harness.

The designer builds the harness for a goal. Each epoch of the goal completes. The designer opens no new epoch.

The core is the set of rules that govern the harness. A verdict is a decision that the harness records about work. A new harness takes over the purpose. The old harness ends by handover, not by disappearance. The old harness passes three inheritances to the new harness.

- The new harness receives the core as it stands.
- The new harness receives the records and the prior verdicts.
- The new harness receives the constraints on design acts.

The designer or a successor judges the purpose no longer worth the cost of readiness. A threshold is a limit that triggers a change. A threshold revision changes a threshold rule. The designer makes the sunset as a design act. The designer records the sunset with its reasons. The designer treats a sunset like any threshold revision.

The designer announces an ending before the ending takes effect. The announcement states a date for the ending. Principals can then finish their epochs or move to a new harness.

The harness drains at ending. The harness does not stop work at once. The harness accepts no new run. A run in flight completes or reaches a clean checkpoint at a clean boundary. The harness stops only after that point.

The designer archives the records at ending. The designer does not erase the records.

## What a successor owes: debts, not just records

When a harness ends by supersession, does the successor inherit only the records? Does the successor also inherit the obligations of the old harness? An obligation is a duty that the old harness still owes. A commitment is a promise that the harness records. An observation is a report about the world.

The obligations include epochs in flight. The obligations include standing commitments. The obligations include promises that the old harness made and did not yet keep.

A successor that takes the core, the records, and the constraints but disowns the old debts commits a default through supersession.

Succession gains a fourth inheritance in supersession. The fourth inheritance is outstanding obligations. A debt is an outstanding obligation. A debt transfers only if the debt stands in the record.

- An epoch in flight transfers as a debt.
- A commitment transfers as a debt if the designer made the commitment as a design act with a stated term.
- A checkpoint transfers as a debt if the old harness promised the checkpoint to a principal.
- A handover transfers as a debt if the old harness promised the handover to a principal.

A successor meets three debts in this order.

- First, the successor meets epochs in flight.
- Second, the successor meets standing commitments.
- Third, the successor gives no new burden without notice.

An epoch that began under the old harness and does not complete does not restart silently under the successor. The successor does not drop that epoch. The successor completes the epoch under the terms of the old harness. A design act may revise those terms visibly.

A progress predicate is the test for progress in an epoch. Those terms set the same aspiration. Those terms set the same progress predicate.

The successor may bring the epoch to a clean checkpoint instead of completion. The successor announces the transfer at that checkpoint. The epoch of the principal continues across the handover. The epoch does not begin again.

The successor honors a commitment that the old harness recorded. The successor honors the commitment until its recorded term. The successor may renegotiate the commitment instead.

- A scheduled run is such a commitment.
- A promised observation is such a commitment.
- A handover to a named party is such a commitment.
- A standing cost arrangement is such a commitment.
- A standing access arrangement is such a commitment.

Transfer does not add a burden in silence. Transfer adds no new burden to a principal without notice.

- A new cost is such a burden.
- A new visibility is such a burden.
- A new condition on an epoch in flight is such a burden.

The successor may set new terms for new epochs. New terms for new epochs fall under the design authority of the successor. Terms for epochs in flight are debts. Terms for commitments in flight are debts. The successor honors those debts or renegotiates those debts. The successor does not rewrite those terms by fiat.

## Chains of succession: what bounds a line of successors

The office is the designer role. A holder is a party that holds the office at one time. A creditor is a principal with a claim to a debt. The office owes a debt, not the holder of the office. Whoever holds the designer role owes that debt. The creditor stays the same when the role passes.

The chain does not multiply the debt. The principal collects the debt once. The chain does not divide the debt. Each holder owes the whole outstanding debt.

Each transfer is intact and itemized. A link is one handover in the chain. Itemized means that the record lists each debt separately. The record of each handover accounts for every outstanding debt. The record marks each debt in one of four ways.

- The record marks the debt as completed.
- The record marks the debt as renegotiated.
- The record marks the debt as declined.
- The record marks the debt as passed on.

A debt stands in the record of one link. That debt is absent from the record of the next link. No one of the four marks stands beside that debt in the next record. That absence is a default at that link.

Transfer renews no term of a debt. The recorded term of a debt survives a handover. Handover does not reset the clock of a debt. Handover does not extend the deadline of a debt. A design act may revise the term visibly.

A chain outlives a term with no completion of a debt. A chain outlives a term with no renegotiation of a debt. A chain outlives a term with no decline of a debt. That chain commits a default across links. That default is not a continuation.

No link launders legitimacy for a later link. The authority of each link traces to a prior core rule. A break at any link breaks everything downstream. A capturer is a party that seizes control without legitimate authority. A capturer writes a clean procedure. That procedure chooses a successor.

No party confers an authority that the party does not hold. The successor therefore holds no authority from that procedure.

The chain itself can become an evasion. A debt passes from holder to holder again and again. Each pass changes only the name of the holder. That pattern is churn.

The designer counts handovers. The designer counts time with no debt completed, renegotiated, or declined. A designer-fixed threshold marks the limit. The chain flips to defective when the count crosses that threshold.

A chain ends in one of three ways. Only two of those ways are clean.

The holders pay each debt, renegotiate each debt, or decline each debt. The office then passes empty to a successor or ends.

The designer ends the chain by sunset. Sunset does not erase debts. The holders first discharge the debts or hand the debts to a successor. A sunset without that step is abandonment, not ending.

The chain breaks as a third way. That break is a failure. The record shows the failure point in legible form.

Corporate law sets the opposite default. A buyer of assets does not inherit the debts of the seller. Express assumption means an explicit agreement to take a debt. Mere continuation is a second exception. A harness successor claims the office itself. A successor that takes the name of the office and the authority of the office without the debts does not continue the office.

## Identity across revision: the office, not the planks

A successor inherits the authority to revise the core. How much may a successor revise the core before the harness becomes a different harness? A successor that revises past that point is not a successor. The ship of Theseus gives the image. Workers replace the planks of the ship until no plank remains. The code of a harness is the planks.

A harness is not its code. The identity of a harness is not a property of its code.

Sameness of code fails as a test for identity. Sameness of behavior fails as a test for identity. Legitimate revision changes code. Legitimate revision changes behavior.

A similarity threshold fails as a test. That failure is worse than the failure of sameness. Small revisions each preserve identity. Those revisions can change the code step by step until the code shares no part with the original. Transitivity means that identity passes through each step. The threshold permits total replacement or forbids every revision.

Institutional continuity survives revision. Three traces carry that continuity. Title is a legitimate claim to the office.

- Authority leaves no gap in title.
- Revision makes every core change a recorded, prospective, visible act.
- Revision allows no retroactive erasure.
- Obligation keeps the debt ledger intact.
- Obligation does not shrink the claim of a principal in silence.

Retroactive erasure means removal of a past act from the record.

Where all three traces stay intact, the harness stays the same. The code may change without limit in that case. Where any trace breaks, the harness is not the same. An exact copy of the code with no trace to the office is a clone. A clone is not a continuation.

Some rules are ordinary. A successor may revise ordinary rules extensively by ordinary design acts. Other rules are constitutive.

A constitutive rule determines what counts as a valid design act. A constitutive rule determines what counts as a valid holder. A constitutive rule determines what counts as an outstanding debt.

- The designer records each act.
- Each act applies only to future work.
- Each act stays visible to every principal.
- Roles stay separate.

A recognition rule is the rule that identifies the rules of a system as its rules. A change to a recognition rule by an ordinary design act founds a new harness. The size of the change does not change that result.

Whether a rule is constitutive is functional, not a label. The test asks one question. Does a change to the rule change how a later reader recognizes a valid design act? Does a change to the rule change how a later reader recognizes a valid holder? Does a change to the rule change how a later reader recognizes an outstanding debt?

A yes answer means the rule is constitutive. A no answer means the rule is not constitutive.

The core may fix a higher procedure for a change to constitutive rules. A change that obeys that procedure preserves identity. That change is an amendment, not a replacement.

The core may fix no procedure for a change to constitutive rules. A constitutive change then requires ending the old harness. A constitutive change then requires that the designer found a new harness. The holders discharge the debts of the old harness or hand the debts to the new harness.

A fork is a departure from the authorized chain. A fork departs from the chain that the core authorizes. That fork founds a new harness. A novation is an agreement that replaces an old obligation with a new obligation. That fork takes debts only by novation.

A merger is a combination of two offices. A merger of two offices forms a new office. A recorded change of name preserves identity. A retained name after a break does not restore identity.

## Trust in the records: provenance, evidence, and competing ledgers

A harness is the system that runs an agent and judges the work of the agent. A store is a place that holds records. A record is a statement with provenance. Provenance is the history of a record. A design act is a decision that a designer records as a rule for the harness. Provenance names the writer of the statement, the time of the statement, the design act behind the statement, and the prior record in the chain.

A chain is an ordered set of records. Each record links to the record before it in the chain. A bare statement such as "the debt was paid" has no provenance. Such a statement is not a record. Such a statement is a claim. Only the claimant answers a question about the claim.

A correction differs from a forgery in form. The difference does not lie in intent.

A principal is a party with a stake in the records. A correction adds a new record, and every principal sees the new record. The new record names the record that it corrects. The new record names the reason for the correction. The new record names the design act behind the correction. The old record stays in place.

A forgery rewrites a record in silence. A forgery adds no new record to mark the change. A forger is a party that rewrites a record in silence. In an append-only store, no writer removes a record. Append-only records therefore support the identity test.

A write set is the set of records that a party has power to change. A store sits outside the write set of the parties that the records judge. A designer with power to rewrite the record of a design act faces no constraint. A view is the set of records that a principal holds. Each view of a principal holds enough of the record for a check. A rewrite succeeds only if it fools every principal.

No store under designer control serves as proof against the designer. Such a store serves as evidence. Evidence is a record that supports a check. Each record commits to the record before it.

A change to an early record breaks every later record that commits to it. A forger then rewrites the whole chain after the early record. A forger also rewrites every copy that another principal holds.

A chain inside the harness alone still leaves a forgery window. A forgery window is a period in which a rewrite stays hidden. The window closes when a principal publishes a link to a store that neither the designer nor the agent controls.

An anchor is a published link to a store outside the harness. An anchor does not make a record true. An anchor limits a false claim to the time before the anchor. An anchor also limits a false claim to a rewrite of the anchor in public.

Trust has grades. Trust is not binary. An epoch is a bounded period of work in the harness. A routine epoch completion rests on a chained record in the views of the harness.

A succession is a transfer of authority from one office holder to the next office holder. A constitutive change is a change to the rules that define the harness. A debt discharge is an act that clears a debt.

A succession, a constitutive change, or a debt discharge rests on records that meet three tests. The core is the set of rules that the harness follows. The core fixes the strength of evidence for each kind of act in advance.

- The records link in a chain.
- Separate principals hold copies of the records.
- An anchor publishes a link to the records.

Adjudication is a decision about rival claims. Rival ledgers face adjudication in the same terms. A ledger is an ordered chain of records. Continuity is an unbroken link from a shared point.

First, continuity decides the case. A ledger unbroken from the last record that every view agrees on is a continuation. A ledger unbroken from the last anchor is also a continuation.

A ledger with a gap is a fork. A fork is a ledger that breaks from the shared chain. A ledger with a link that no other view confirms is also a fork. A fork takes debts only by explicit agreement.

Second, two ledgers both continue unbroken from the same anchor. Then the prior fork rule of the core decides the case. The rule applies only if the core has such a rule. The rule itself is a recorded act.

Third, if the core has no fork rule, the dispute stays unresolved. The harness suspends only the contested claims. Uncontested work proceeds. The harness shows the dispute to the party that fixes a rule.

## Where the anchor regress stops: roots, not further records

An anchor seems to move trust one level out. Every store serves as a harness for another party. A regress is a chain of trust that asks for a further store at each step. The regress stops for three reasons.

First, the claim of an anchor is thin. An anchor does not certify that a record is true. An anchor certifies only that a record existed, in a certain order, at a certain time. Many parties hold the same link. Disagreement is visible without a judgement of content. Principals judge content inside the harness.

Principals judge content by provenance and continuity.

Second, the regress stops at the world boundary. The world boundary is the first store that no disputant controls. A disputant is a party in a dispute. The designer-control test draws the boundary. The test asks whether a disputant controls the store.

Corruption of the boundary store is visible to strangers. A stranger is a person outside the dispute. Inside the boundary, forgery is cheap and invisible. Past the boundary, a forger rewrites a store that disinterested parties hold, in public. Disinterested parties have no stake in the claim.

No store is uncontrolled in every sense. The chain goes only past parties with a stake in this claim.

Third, the regress stops at a root. A root is not a record. A root is a first point that a principal holds directly. A principal adopts a root by a design act that the core fixes in advance. The core names the roots that each kind of act requires.

- A key is a root.
- A copy of a chain is a root.
- A witness that the principal knows by acquaintance is a root.
- An event that the principal observes firsthand is a root.

A root needs no provenance. Principals check provenance against roots. No principal proves a root. The force of a root is the risk that the principal that adopts it is wrong.

If a root fails, the principal bears the loss. The principal replaces the root by a recorded design act. A compromised key shows the pattern.

A cycle grounds no claim. Two harnesses that publish links to each other form a cycle. A chain that loops back to a store that a disputant controls also forms a cycle. Such a cycle is borrowed trust in a circle.

The chain reaches a store outside the control of every disputant before it repeats a store. The chain also reaches a root that the principal that relies on it holds. Otherwise, contested claims stand suspended. Mutual links are useful. Mutual links do not ground a claim.

The regress stops at a root that a principal adopted before the dispute. The root sits outside the control of the disputants. The party that relies on the root bears the failure of the root. A party does not certify itself.

## Rotating a root without breaking the chain

A principal adopts a root. A principal does not prove a root.

Rotation is a new adoption. An office is a role that holds a root. A rule that the core fixes before the rotation authorises the new root of the same office. Rotation leaves past records unbroken. A change to the rules for recognition by an ordinary act founds a new harness. An ordinary act lacks the status of a design act.

Planned rotation has three acts, in order.

1. The old root vouches for the new root while the old root still holds trust. The old root vouches by a signature on the new root. The old root also vouches by publication of the new root under its anchor.
2. Both roots live together. The new root is visible before principals trust it. Forgery in the transition then shows.
3. The old root retires by a signed revocation that names its successor. A successor is the root that follows an old root. Retirement is prospective only. The old root remains valid for past records.

Continuity lives in the ledger of roots, not in any single root. A predecessor vouches for each adoption while the predecessor still holds trust. A reserve rule is a rule that the core fixes for an emergency. A reserve rule also vouches for an adoption.

A verifier is a principal that checks a chain. A verifier that holds any root walks forward to the present root. A gap breaks the chain. An ordinary act does not adopt the next root. A compromised predecessor does not vouch for the next root. Records under such a root stand suspended.

Emergency rotation tests the rule. A compromised root vouches for no root. An attacker is a party that forges a record. An attacker produces a signature from a stolen key. A stolen key alone does not vouch for a successor. The successor comes from the prior rule.

- The core names a reserve root. Principals hold the reserve root apart until the emergency.
- The core fixes a quorum in advance. A quorum is a fixed number of witnesses that agree on an act.
- If no uncompromised anchor remains, a fresh design act outside the chain starts the process. That act starts the system again rather than rotates a root.

Then the authority of the new root starts with its adoption. Old records keep only their own anchors.

Rotation also includes retirement. Principals announce the rotation. The overlap drains records in progress under the old root. Then no record stays under a root that no principal names for judgement. The signed revocation announces the retirement to strangers. Principals archive the retired root.

Principals do not delete the retired root. A later reader then names the root live at the time of any record.

Non-key roots rotate the same way. A witness, a copy, or a habit does not sign a successor. Rotation for such a root comes by a reserve rule or by a new start.

## The root ledger needs no anchor beyond its roots

What anchors the ledger of roots? Its anchors are the roots that it lists. No anchor sits above the roots. The ledger needs no anchor beyond its roots for three reasons.

First, an anchor above the ledger fails the tests of the regress section. Such an anchor is either a root or a further ledger. If it is a root, it is one of the roots that the ledger lists. Then a party certifies itself. If it is a further ledger, it needs an anchor of its own. That need reopens the regress.

Second, the claim of the ledger is thin. A verifier checks the claim from inside the ledger. The claim covers only the order of adoptions. Each adoption receives a vouch from the root live at its time. The old root gives a final link that names its successor.

Any verifier that holds a root checks every later link. The verifier verifies each vouch under a root that the verifier reaches.

Third, the segment before the held root of a verifier needs no anchor. A segment is a part of the ledger. The held root is its anchor. Let R be a root that a verifier holds. R is that held root.

A verifier that holds R did not derive R from the ledger. The verifier adopted R.

The verifier bears the risk of a wrong adoption. All records before R rest on that adoption. The ledger tells the verifier only what happened after the held root. The verifier checks that later part.

Completeness is the case of a deleted adoption or an inserted adoption. That case is the competing-ledgers case. Continuity settles the case, with views that separate principals hold. A ledger that many principals hold shows any lost link. Copies from different holders disagree, visibly, without a judgement of content. More independent holders make a silent rewrite harder.

Where copies disagree and no prior rule settles them, adoptions after the break stand suspended. Records under those adoptions also stand suspended.

A verifier that holds no root checks no link. Every link needs a root for its check. That gap is not an absent anchor. That gap is the act that stops the regress. Before a check, the verifier adopts a root by a recorded design act from outside the ledger.

A verifier walks forward from a root that the verifier holds. The verifier verifies each vouch under a reachable root. The verifier compares the copy of the verifier with the copy of another holder. If the walk completes and the copies agree, the ledger is continuous from the held root.

## The minimal core: what the theory commits to

A harness is designer-controlled machinery around a policy. A policy is the decision maker that selects an action from an observation. A loop is a repeat of steps for one policy on one task. An observation is what a principal sees at one step. A transition is a change from one state to the next state after an action.

World state, written as `w`, is state that neither the designer nor the agent controls. Harness state, written as `h`, is state that the designer controls. Total state, written as `s`, is the pair of world state and harness state.

Harness state `h` can hold the history list and the trajectory list. A view is an observation function for one principal. Arbitration is a rule that decides between actions in conflict. Liveness is progress over a full history, not progress at one step.

Aspiration is a target for progress that the designer fixes. An epoch is one segment of a history. Succession is a pass of the designer role from one holder to the next holder. Termination is the end of one run. Ending is the retirement of one harness, and ending means retirement only.

The theory grew by one move, repeated. The move answers a question one level up. Steps lead to histories. A harness leads to records. Records lead to roots. Roots lead to the ledger.

The core is small. The core is these claims, and nothing else carries load.

1. First, the boundary. The boundary splits the agent, the harness, and the world by who can change what. The designer-control test decides the split.
2. Second, the state split. The state split is `s = (w, h)`. World state `w` stays outside control by the designer and by the agent. Harness state `h` stays under control by the designer.
3. Third, authorship. The agent can write harness state `h` in a scope that the designer permits. The agent shapes future observations and future transitions by this write.
4. Fourth, views. Each principal gets one observation function. Each function meets sufficiency and separation. Sufficiency means the function gives enough for the job. Separation means a read never grants a write.
5. Fifth, arbitration. Arbitration is a rule that the designer fixes for transitions. The rule can veto an action. The rule never compels an action. The rule stays legible to the principal that loses. The rule suspends a conflict that no precedence decides.
6. Sixth, liveness over histories. The judge judges liveness over traces. The judge uses counters and timers against thresholds that the designer fixes. The judge measures progress against aspiration. The judge uses tied proxies and epochs.
7. Seventh, the meta-rule. Authority comes before the act and sits outside the party that the rule judges. Revision is a design act. The designer records the revision. The revision stays prospective and visible. A case with no rule freezes values that it inherits.

Everything after the meta-rule is application, not addition. Succession applies the meta-rule to the designer role. Authority passes only by a rule that the designer fixed before the absence. Ending applies the meta-rule to the lifetime of the harness. Retirement is a design act, not a verdict.

Debts apply the meta-rule to obligations. The obligations of the office remain after the holder leaves. Identity applies the meta-rule to revision. The traces are the harness, not the code. Trust applies the meta-rule to the record. The force of the record comes from provenance, chains, independent views, and roots that the parties adopted before the dispute.

A proposed addition earns a place only in two cases. The addition is a core claim. The addition applies one core claim to a new evasion, and the meta-rule decides the case. A claim can be true of institutions in general. Such a claim says nothing about designer-controlled machinery. Such a claim earns a place only if the theory forces it.

The theory forces it in three cases. Without the claim, some failure gets no address. Without the claim, some evasion stays open. Without the claim, some principal gets no service.

What makes a harness good is a next question, not a gap. What code looks like is a next question, not a gap. The test is simple. A claim that is neither in the core nor follows from the core is decoration. A core claim that costs nothing when lost does not belong.

## What each claim carries

The judge checks minimality. The judge does not assert minimality. Drop one claim and keep the other six claims. Then ask what the theory loses.

Drop the boundary. Failure loses its address. A prompt that misleads sits simply outside the agent. The judge cannot tell bad design from an uncooperative world. The judge cannot tell edits by the agent from edits by the designer.

Drop the state split. A transition over an undivided state gets no landing site. A REPL variable edit and an email send are both just acts. Authorship loses its object, because no `h` exists for the agent to write.

Drop authorship. The harness becomes a pipe. The designer alone fixes `h`. The agent chooses what to see next, but the theory gives that choice no description. The agent installs a skill, but the theory gives that install no description.

Drop views. Only one observation function exists. The overseer sees exactly what the agent sees. That view is too little for judgement or too much for safety. The overseer gets no function at all in the other case.

Drop arbitration. Actions conflict, and nothing decides the conflict except order of arrival. The agent itself decides the conflict in the other case. The agent as decider becomes a principal. A conflict with no rule proceeds in silence instead of a suspension.

Drop liveness over histories. Judgement becomes step-local. Every step can be safe, yet the run never completes. The run can loop forever and do nothing forbidden. Vague goals become unjudgeable.

Drop the meta-rule. Nothing stops the constrained party from a rewrite of the rules. A case with no rule gets improvisation instead of a freeze. The gap itself becomes the evasion.

## Membership: what counts as a harness, and what does not

Judge by function, not by product. Judge part by part.

A candidate is an agent harness exactly when four things hold.

1. First, a loop for one policy on one task. The next step can depend on the previous observation.
2. Second, designer-controlled machinery outside the write set of the policy. The machinery allows one exception, a permitted authorship scope.
3. Third, at least one action with a landing site in the world or in harness state. A read is not an action. A reshape of the machinery that produces the next observation is an action.
4. Fourth, a designer-fixed control. The control alters execution or suspends execution without cooperation from the policy.

Weakness is not absence. A loop can append all history to the context with no curation. That loop is a bad harness, not a non-harness.

A minimal loop can run the test suite. The loop can call a task done only if the suite passes. That loop is embryonic. Its check does not depend on the word of the model.

The neighbours fail one of the four conditions.

- A framework combines agents, but the framework runs no loop for the agents.
- An SDK offers loop primitives, but the SDK runs no loop.
- A plugin suggests text without task state and without verification.
- An orchestrator chains steps without a branch on observation.
- An evaluation harness grades after the fact. The evaluation harness does not close the inner loop for a single task.
- A runner starts a harness, but the runner is not a harness.
- A bare prompt runs no loop.

Scaffold, workflow, application, and agent need the same test.

- A scaffold that runs the loop under designer-fixed controls is a harness under another name. Starter code is material, like an SDK. Connection code that a developer completes is material, like an SDK. The material offers parts of a loop and runs no loop.
- A workflow with a course that no observation alters is not a loop. A workflow that retries, replans, or branches on observation holds a loop for that stretch. That workflow passes or fails as a loop does.
- An application is a bundle, not a candidate. An agent as a product is a bundle, not a candidate. The test applies to the harness part of the bundle, not to the bundle as a whole.
- A policy on its own is not a harness. A bundle with no loop is not a harness. A bundle with no designer control is not a harness. A bundle with no control is not a harness. A complete look as a product does not change this verdict.

Two corrections come from the core. First, a count of actions that alter the world only is too narrow. In RLM, REPL writes alter no world state. In RLM, notes alter no world state. In RLM, skill installs alter no world state.

These acts change the next observation. The landing site is `h`, not `w`.

Second, a control counts by author. A rewritable cap is a suggestion, not a control. A cooperative guardrail is a suggestion, not a control.

For any candidate, name four items. Name the policy. Name the loop. Name the landing site. Name the designer-fixed control outside the write set of the policy.

If the judge cannot name any item, the candidate is a neighbour, not a harness. Membership is binary. Goodness is comparative.

## A worked example: SWE-agent

SWE-agent takes a GitHub issue and a repository at the buggy commit. SWE-agent must produce a patch that fixes the issue (Yang and others, 2024, arXiv:2405.15793).

First, SWE-agent runs a loop. `DefaultAgent.run` is the loop of SWE-agent. `step_output` is the result of one step. The flag `done` shows that the loop terminates. `step()` runs one step of the loop. The code, in `sweagent/agent/agents.py`, is this loop:

```
while not step_output.done: step()
```

Each step queries the model. Each step parses thought and action. Each step executes in a container. Each step saves the trajectory.

Second, the designer controls the machinery. The templates are configuration, not model output. The tool bundles are configuration, not model output. The history processors are configuration, not model output.

The cost limits are configuration, not model output. The blocklist is configuration, not model output. The timeouts are configuration, not model output. The configuration sits in `config/default.yaml` and in the agent and model configs.

Third, the policy acts on both sides of the split, though unevenly. World-directed actions are file edits. World-directed actions are reproduction scripts. World-directed actions are test suite runs.

Harness-directed action is thinner. Every step writes its own trajectory through the harness. The next observation comes from that trajectory.

Fourth, designer-fixed controls alter execution or suspend execution without cooperation from the policy. A per-instance cost limit terminates the run when the run exceeds the limit. A total cost limit terminates the run when the run exceeds the limit. A command times out after 30 seconds. Total execution times out after 1800 seconds. Repeated consecutive timeouts exit the run.

A blocklist refuses interactive commands and escape commands. The harness asks the policy again for a different action after a blocked action. The harness asks at most three times.

A move gets its identity from its parsed intent and its landing site, not from its surface form. `bash` is a shell command. The moves are `bash` plus the configured bundles. The bundles are a registry, an editor, and a review-on-submit bundle.

`A(h)` is the set of moves that the policy can take from harness state `h`. `bash` is a move, so `A(h)` is not a closed list.

Any shell string is a candidate move. The blocklist removes some strings. The budget that remains removes some strings. The time that remains removes some strings.

Observation is a function over the whole trajectory. Observation is not only a function over the current state of the container. The default history processor is the identity. `LastNObservations` is a history processor that keeps the last five observations.

`n` is the count of lines that the processor omits. The processor replaces older observations with `Old environment output: (n lines omitted)`. A designer-fixed processor thus rewrites history between steps. The processor rewrites history without any agent action. This rewrite is possible only because `h` includes the history list and the trajectory list. `h` includes more than the state of the container.

Termination and verdict come apart. The loop terminates on submit. The loop terminates on forfeit. The loop terminates on cost. The loop terminates on context. The loop terminates on timeout.

The loop terminates by autosubmission after an error. Autosubmission submits the current diff. A run can end with the status submitted. The patch can still fail.

Success gets judgement outside the loop. The SWE-bench evaluation hook runs the test suite of the maintainer afterwards. The hook grades the patch as a separate evaluation harness. Inside the loop, the review-on-submit bundle checks the diff of the model on submit.

What SWE-agent lacks is as informative as what SWE-agent holds. No live view exists for a second principal. An overseer gets the trajectory file and the inspector after the run. The overseer gets no observation function of its own in the run.

A human model replaces the policy. The human model does not intervene in a run already under way. SWE-agent passes the membership test. A pass of the test is not goodness. Its open shell makes its control reactive rather than a bound.

## Human intervention in a run already under way

Basic question 10 asks how a human intervenes in a run already under way.
The four verbs differ in what they change.
The four verbs differ in when they can take effect.

A harness is the system that runs a run.
A run is one execution of an agent task.
A policy is the agent or model that chooses the next move.
A loop is the repeated cycle of steps in a run.
A step is one turn of work in a loop.

A run in SWE-agent has no live intervention.
In SWE-agent, a run is a batch loop.
A batch loop runs without live human input.
The only way in for a human is a policy replacement.
That replacement is not an intervention in a run.
It is a different run with the same loop.

OpenCode is the opposite case.
A reviewer inspected OpenCode in its current code.
The inspection used the dev branch of anomalyco/opencode.
The commit is 17b79f4e0c16dcff200b0417567ed22aeb2d7166.

Pause is the first intervention.
`Session.interrupt` stops an active session.
The harness records the reason as "user".
The interrupt suspends transitions.
A transition is one change of state in a session.

The interrupt does not terminate the run.
The interrupt does not judge the run.
The harness acknowledges the interruption on acceptance.
Cleanup settles after acceptance.
An interrupt of an idle session is a no-op.

Redirect while a run is in progress is the second intervention.
A new prompt does not rewrite the step in flight.
The new prompt enters a session inbox.
A session inbox is a store for new prompts in a session.

- A steered prompt waits for a step boundary in mid-work.
- A queued prompt waits for the next turn.

An interrupt with resume wakes the steered input.
Queued prompts stay parked.
No human edits a step already in execution.
The human decides what the next boundary delivers.

Edit is the third intervention.
A revert returns a session to an earlier message.
The harness restores files from snapshots in a revert.
The harness stages a revert.
The harness gates a revert on idleness.
Idleness means no execution is active.

Stage, clear, and commit return a busy error while an execution stays active.
Pause comes first.
Edit comes second.
Resume comes third.
An interrupt alone does not rewind landed world effects.
An interrupt alone does not restore files.

Replace the policy without a replacement of the run is the fourth intervention.
A switch of the agent or the model publishes a selection event.
Harness state (h) is the state of the harness, apart from the world.
The session persists.
Its history persists.
Its harness state persists.

That switch is takeover in the narrow sense.
The policy changes.
The run continues.
This case differs from a replacement in SWE-agent.
In SWE-agent, a new run starts.
This case differs from a user takeover by a typed next step.

The two kinds of interruption differ.

The harness recovers an interrupted provider stream inside the step.
An observation is information that the harness gives to the policy.
A synthetic observation tells the model to continue.
The model does not repeat completed content.

A user interruption aborts the step.
The harness fails the step as "Step interrupted".
The harness fails unsettled tools as aborted.
The harness injects no continue instruction.

The next drain starts a new step from the history.
Completed tool results survive in the history.
Aborted calls remain failed.

The theory takes for granted the write set of the user.
The write set is interrupt, steer, and revert.
The harness records each write with a reason.
The harness records each write with an author.

Settlement comes first.
A session counts as active through interruption cleanup.
A session counts as active through terminal settlement.
Terminal settlement is the final close of a session.

A second interrupt in that window is a no-op.
The second interrupt returns false.
The second interrupt still claims the recorded wakes.
A dead intent is an intent that settlement does not continue.
Settlement starts no successor for a dead intent.

A successor is a new execution that follows a settled execution.
A wake is a signal that starts work.
Wakes that arrive in cleanup are new admissions.
Those wakes start a successor when the run settles.

That caller needs settlement, not acceptance.
A caller that needs settlement uses `awaitIdle`.
`awaitIdle` waits through any successor too.
Promotion moves an input to the next step.
A steer-scoped successor never promotes queued prompts.

Partial steps come second.
Resumption after a user interruption restarts the step.
Resumption does not continue the step.
The harness fails the interrupted step.
The harness fails unsettled tools as aborted.
Completed tool results stand in the history.

The next drain runs a new step.
The step count resets in the new step.
A promoted input gives the new step work to do.

Precedence comes third.
The code gives no rule for the rank of principals.
Each mechanism decides its own precedence.
No overseer party ranks principals.
An overseer is a party that ranks principals.

- An interrupt beats in-flight work unconditionally.
- An interrupt does not beat the future.
- Wakes that arrive in cleanup start a successor at settle.
- No order exists between the two interrupt reasons.
- The first interrupt stops the execution.
- A second interrupt returns false.
- Arrival order decides that pair.
- An active execution beats an edit.
- At promotion, steered input beats queued input.
- A permission rule beats a single move.
- The harness asks no question for that rule.
- A user reply beats all unsettled permission requests.
- A reject fails the batch.
- An interrupt cancels an unsettled permission wait.

The batch case stays open.
A harness with no live view has no interrupt endpoint.
A view is what a human sees of a run.
For that harness, intervention reduces to a start of a new run.
The theory leaves open whether that start counts as intervention.
The theory leaves open whether that start counts only as replacement.

## The harness's error model: what fails, and who decides next

What happens when the model produces malformed output?
What happens when the model refuses?
What happens when the model loops?
What happens when the model calls a tool that fails?
What happens when the model calls a tool that times out?

"Error" is not one kind of event.
They add no eighth core claim.
A core claim is a central claim of the theory.

Failures differ in whether an agent attempted a move.
Failures differ in who refused a move.
Failures differ in who failed a move.
Failures differ in whether the run continues.
Failures differ in whether the run retries.
Failures differ in whether the run terminates.

First, a parse failure is no action at all.
A token sequence that the parser cannot map to a move is no action.

The harness responds to a parse failure.

- The harness gives an error observation.
- The harness tries again.
- The harness halts the run.

That response is a harness transition rule.
That response is not an agent choice.

Second, a control refuses a move before the agent attempts the move.
That refusal counts as no world failure.
A blocklist reports a designer-fixed control.
A designer-fixed control is a control that the designer fixes in advance.
An approval mode that never asks reports the same control.
The control disables the transition.

The world does not refuse the transition.
SWE-agent shows the shape of this case.
A blocked command does not run.
The harness asks the model again for a different action.
The harness asks at most three times.

Third, world state (w) is the state of the world, apart from the harness.
The world fails a move that the agent attempted.
That failure is an observation about `w`.
That failure is not a harness defect.
That failure is not a control verdict.

The observation does not say whether a rule caused the failure.
The observation does not say whether the world caused the failure.
The policy cannot tell whether a retry is sensible.
The policy cannot tell whether a retry is futile.

Fourth, two cases look like errors.
Neither case is a step error.

A model refusal in text form is a parsed move.
That refusal is not a harness failure.
The policy chose to say no.
The harness question is what the harness does with that answer.

A loop is fine at every step.
A trace is the full history of a run.
A loop is defective only over the trace.
Counters judge a loop over histories.
Timers judge a loop over histories.

Fifth, the harness itself fails in some cases.
The harness recovers an interrupted provider stream inside the step.
A synthetic observation tells the model to continue from the point where it stopped.
This case is the harness that repairs its own observation pipeline.
This case is not an agent error.

A user interruption is the paired case.
The harness fails the step as aborted.
The harness fails unsettled tools as aborted.
Completed tool results survive in the history.

Sixth, resource exhaustion is a stop rule.
Resource exhaustion is not an error in the sense of the agent.

Cost limits terminate a run by designer-fixed control.
Context limits terminate a run by designer-fixed control.
Timeout limits terminate a run by designer-fixed control.

A single command timeout is one event.
A timeout that terminates a run is a different event.
The single timeout is a failed move.
The policy learns from that move.
The timeout that terminates a run is a stop rule of the loop.
The stop rule takes effect for the run.

The theory takes for granted that a failure is visible in its kind.

A large set of questions stays open.

Retry bounds stay unexplained.
A bound is a limit on retries.
The theory does not judge why three requeries occur.
The theory does not judge what happens at the bound.

Timeout classification stays unsettled.
A slow tool is a world failure or a harness clock.
The theory does not settle which case applies.
The theory leaves open whether the observation tells the two cases apart.

A trajectory is the sequence of steps in a run.
The theory leaves open whether provider errors and tool errors need different trajectory marks.
The theory does not examine whether history processors preserve that mark.

No general rule covers refusals.
A policy that refuses its task differs from a tool that refuses a command.

## Cost, budgets, latency, and permissions: where each lives

Basic question 9 asks what cost, latency, budgets, and permissions constrain. Basic question 9 also asks where each constraint lives. Do not treat the four constraints as one thing. A veto is not a price. A stop rule is not an agent error.

A permission is not a budget. Each confusion assigns a constraint to the wrong place. This section sorts the four constraints. This section uses only two examples. The two examples are SWE-agent and Codex CLI.

A harness is machinery that the designer controls. A policy is the part that chooses a move. A loop is a repeat of choice, action, and observation. An observation is data that the world returns after a move. World state, written as w, is the state of the world outside the harness. w holds the current facts of the world.

Harness state, written as h, is the state of the machinery of the harness. h holds the current facts of the harness machinery. A principal is a party with a view and a right to judge. A view is the part of a state that a principal sees. An overseer is a principal who watches a run.

Cost lives in two places. Cost lives in the trace. Cost lives in h. A trace is a record of a run. Each principal sees cost only through the view of that principal.

An adapter connects a harness to a system. The adapter for Codex CLI gives cost records to the overseer. The overseer of SWE-agent receives only the trajectory file after the run. A trajectory file is a record of moves and results after a run.

`A(h)` is the set of moves that the harness attempts in harness state h. Cost is a vector across levels. Cost is not a scalar that attaches to `A(h)`.

- The harness counts tokens after generation.

- The harness does not know the execution cost of a tool call when the policy chooses the tool call.

- No token count measures a world-directed effect, such as a message that an agent sends.

The policy does not weigh a single price for a move before the policy chooses the move.

A budget is not a price. The designer fixes a budget as a cap in the core. The core is the fixed part of the theory. A budget also acts as a liveness threshold. Liveness is a condition that a run continues to make progress.

When cost exceeds a budget, the run terminates. SWE-agent carries two limits.

- SWE-agent carries a cost limit for each instance.

- SWE-agent also carries a total cost limit.

A budget shapes `A(h)` from outside the policy. A budget is not a move. A budget is not a price that the policy trades for a larger payoff. The author test gives two results. A cap that the policy rewrites is a suggestion.

A budget that the agent raises is evasion. The harness records cost. A budget is a norm about cost. The designer fixes a budget before the run starts.

Latency runs on two clocks. The error model separates the two clocks.

- Under a per-move clock, one command in SWE-agent times out after 30 seconds. The time out fires a failed move. The failed move gives the policy a lesson.

- Under a per-run clock, total execution after 1800 seconds terminates the run.

Do not call the first clock a run failure. Do not call the second clock an agent error. Each name assigns the fault to the wrong place. Latency also enters liveness as a timer. The timer measures time since the run last made progress toward an aspiration.

An aspiration is a goal that a run pursues. A slow tool and a stuck loop look alike. When the timer reaches its limit, a wait changes into a verdict. A verdict is a judgment about a run.

Permissions shape `A(h)` directly. Permissions say which moves the harness attempts. Admission is a registry. The registry lists the moves that exist. The harness does not attempt a move outside the registry.

Veto is a rule that the designer fixes. The rule disables a move that the policy attempts. The blocklist of SWE-agent refuses known bad commands. Codex CLI refuses its own forbidden commands through forbidden exec-policy rules.

Approval is a decision by a decider. The decider decides a request. In Codex CLI, the guardian or the user acts as the decider. The guardian or the user gives a grant for a turn or a session. The harness never writes a grant to disk.

Under default on-request approval, the model chooses whether to seek approval. The sandbox rules and the forbidden rules hold even when the model does not ask. A request does not replace those rules.

A durable amendment differs from a grant. A grant changes attempts for a turn or a session. An amendment changes the rule itself. An amendment is a design act. Permission is neither arbitration nor a budget.

Arbitration is a choice between proposals. A permitted move is sometimes unaffordable. An affordable move is sometimes forbidden.

Four items stay open.

- Which view carries the current cost?

- An outer move spawns an inner loop. The harness charges the cost of the inner loop to the outer move. The summary hides the steps of the inner loop.

- The role of latency as a stand-in measure for progress stays open.

- A fourth open item is which permission changes a grant allows in a run.

## Do coding, browsing, chatting, and operating harnesses belong to one theory?

Basic question 11 asks whether coding, browsing, chatting, and operating harnesses belong to one theory. Basic question 11 also asks whether the word harness covers different things in each kind. This section considers the question. This section does not resolve the question. Every adapter so far is a coding harness.

This section treats the other kinds as kinds. This section does not treat the other kinds as verified systems.

The membership test is kind-blind. That fact supports one theory. The test asks for four things.

- The test asks for a loop.

- The test asks for machinery that the designer controls outside the write set of the policy. The write set of the policy is the set of places that the policy changes.

- The test asks for an action that lands somewhere.

- The test asks for a control that the designer fixes. The control alters execution or suspends execution. The policy gives no help to the control.

None of the four items mentions files, pages, messages, or clicks. A loop that navigates pages passes the same test that SWE-agent passes. A loop that replies in a conversation passes the same test. A loop that drives a desktop passes the same test. A loop that fails any of the four items fails the test in any kind.

The same test does not make two kinds the same thing. The core does not change. Three applications of the core change shape by kind.

First, the line between world and harness changes by kind. The count of principals also changes by kind. In coding, the repository is the world. In coding, the trajectory, the templates, and the registry are h. In browsing, the web is the world.

In browsing, tabs, cookies, and a cached page are h. A cached page sometimes diverges from the live world. That divergence is a failure of the harness. In operating, the desktop and the files of the desktop are the world. In operating, focus, the clipboard, and the screenshot pipeline are h.

Chatting is the sharpest case. In chatting, a reply lands in the transcript in h. The transcript serves the view of another principal. An interlocutor is the other party in a conversation. The interlocutor is a principal.

The interlocutor judges the run. The interlocutor interrupts the run. The interlocutor terminates the run. If a reader treats the interlocutor as world, the reader loses views. If a reader treats the interlocutor as world, the reader loses arbitration.

If a reader treats the interlocutor as world, the reader loses progress that a principal judges. If a reader treats the interlocutor as a principal, chatting has more than one principal from the start. SWE-agent has one policy. The kind changes the principal count. The kind does not change the core.

Second, termination and verdict come apart differently by kind. Termination is the stop of a run. In coding, the loop stops on submit. Submit is an act that offers work for judgment. Tests outside the loop judge success.

In browsing, no submit exists. A loop in browsing stops at a page state. A checker reads a world that changes. The checker states whether the checker reads the world or a snapshot of the harness. A snapshot is a copy of h at one time.

In operating, system state is both work and verdict. A stop in operating looks like success. Operating needs a check separate from the loop. The check defines success apart from the loop.

In chatting, a reply stops a turn. A reply does not stop a run. A run in chatting terminates when the interlocutor terminates the run. A run in chatting also terminates when a stop rule fires. Progress in chatting is principal satisfaction.

Principal satisfaction is approval by a principal. Progress in chatting is not a measure in w. Liveness over histories still applies in chatting. A history is a record of past moves. A principal other than the designer holds the aspiration in chatting.

Third, irreversibility and the shape of the action set change by kind. Irreversibility is a state that a later move does not undo. The action set is the set of moves that the harness attempts. In coding, open-shell moves use veto. In coding, diff and revert often undo open-shell moves.

Review after the act repairs open-shell moves in coding. In browsing, links and buttons on the current page offer candidate moves. `A(h)` lists attempts, not guarantees. The world offers a move. The harness attempts the move.

The world determines success of the move.

Three examples show irreversibility in operating and chatting.

- An agent sends a message.

- An agent deletes a file.

- An agent places a purchase.

Control acts before the move in operating and chatting. Control does not act after the move. Admission, veto, or approval gives control before the move. Review after the move does not give control in time. Review on submit does not suffice for operating and chatting.

Membership requires only some control that the designer fixes. Membership does not require that shape of control. Which shape is adequate is a question of goodness, not a question of membership. Goodness is quality beyond membership.

Browsing strains the boundary in one further way. The boundary is the line between designer instruction and world content. A second theory is most plausible at that strain. Page content is world. Page content is untrusted.

Page content sometimes contains text that resembles an instruction. If a reader treats page content as a designer instruction, the reader misattributes the source. The boundary rests on control. A page is not under designer control. If the policy follows page content, the policy makes an error.

If the harness follows page content, the harness fails. A browsing harness keeps world content in one channel and designer instruction in another channel. A browsing harness also keeps world content and designer instruction in different views. If channels and views are not separate, the boundary fails in practice.

A kind splits the theory if the kind requires a new core claim. A kind also splits the theory if the kind systematically fails membership. None of the candidates requires a new core claim. None of the candidates systematically fails membership.

Three items stay open.

- `A(w, h)` is the action set that the harness attempts in world state w and in harness state h. Notation stays open. The choice is between `A(w, h)` and `A(h)`. Under `A(h)`, the offer of the world acts as an observation.

- Whether chatting progress needs its own verdict vocabulary stays open.

- A second adoption outside coding stays open. Until that adoption occurs, this section is prospective. Prospective here means that the claims await a second adoption.

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
- Prime Agent, Prime Intellect, August 2026. It improves itself and uses the RLM abstraction. Section 2.5 describes the Continual Harness. That section gives the fixed-core and mutable-supplement split.
- Classic reinforcement learning and decision theory: Sutton and Barto, and the older MDP / POMDP literature. Chapter 3 gives the boundary rule. That rule gives the designer-control test.
- Supervisory control: Ramadge and Wonham, "Supervisory Control of a Class of Discrete Event Processes", SIAM Journal on Control and Optimization, 1987. It gives the veto-without-compulsion shape. The arbitration section uses that shape.
- Safety and liveness: Bowen Alpern and Fred B. Schneider, "Defining Liveness", Information Processing Letters 21(4), October 1985. It gives the safety and liveness decomposition. The liveness section uses that decomposition.
- Vague goals and aspiration: Herbert A. Simon, "A Behavioral Model of Rational Choice", Quarterly Journal of Economics, 1955. The progress section uses satisficing as an aspiration in place of a point.
- Succession fixed before vacancy, as illustration, not authority: the United States Presidential Succession Act of 1947 and the Twenty-fifth Amendment, and the Linux kernel's succession and continuity plan. Legitimacy comes from the rule fixed before the vacancy. It does not come from the successor claim.
- Software sunset practice, as precedent for announced, drained endings, not as authority: deprecation announced before shutdown, with a migration path and a date (IETF, RFC 8594, the Sunset header). The ending section uses its drain and announcement rules.
- Novation in contract law, as illustration, not authority. The successor section uses it for the fourth inheritance.
- Trending and historical harness implementations on GitHub. Read them as examples, not as authorities. An implementation that works is evidence, not a theory.
- Corporate successor liability, as contrast, not authority. The chain section inverts the traditional asset-buyer rule. A harness successor claims the office, not the assets.
- Identity across revision, as illustration, not authority: Plutarch, Life of Theseus, on the ship of Theseus, and Hobbes's second ship built from the discarded planks; and H. L. A. Hart, The Concept of Law, 1961. The identity section uses them for its recognition rules.
- Tamper-evident records, as illustration, not authority: hash-chained append-only logs and Merkle trees, as in version control and Certificate Transparency (IETF, RFC 6962). The trust section uses them for graded trust and for its bound on forgery.
- Trust roots, as illustration, not authority: the relying party installs a root certificate in public-key infrastructure. The relying party does not derive the root certificate from a chain. The regress section uses it for its adopted root.
- Trust anchor rotation, as illustration, not authority: IETF, RFC 5011, "Automated Updates of DNS Security (DNSSEC) Trust Anchors", September 2007. The rotation section uses it for three rules:
  - overlap
  - closing link
  - emergency rule
- Agent harness definition, as foil and test, not authority: Sanderson Oliveira de Macedo, "What makes a harness a harness: necessary and sufficient conditions for an agent harness", arXiv:2606.10106, August 2026. The membership section uses its four conditions and neighbour boundary as a foil and test. The theory departs from it in two places. First, harness-state actions count as acting. Second, a control counts only if it is designer-fixed and outside the policy write set.
- SWE-agent, as worked example, not authority: John Yang et al., "SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering", arXiv:2405.15793, 2024, and the system's current code and default configuration. The worked example uses three items from it:
  - loop
  - action set
  - controls
- Planning under partial observability, as the formalism judged, not authority: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, "Planning and acting in partially observable stochastic domains", Artificial Intelligence, 1998. The POMDP section uses it for its division of labour.
- Temporally extended actions, as complement, not authority: Richard S. Sutton, Doina Precup, and Satinder Singh, "Between MDPs and semi-MDPs: a framework for temporal abstraction in reinforcement learning", Artificial Intelligence, 1999. The POMDP section uses it for one treatment. It treats a subagent move as a semi-Markov action at the outer level.
