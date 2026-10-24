# Theory

This states the starting definitions, and the questions they
leave open.

## Starting definitions

Think of a simple partially observable Markov decision process.

- An agent is a policy `pi(a | o)`, where `o` is the observation,
  in the LLM case the full context history.
- An action `a` is, at the lowest level, a token or log-probability
  the network generates.
- The harness is part of the environment. It carries out:
  - the transition `p(s' | s, a)`: taking actions in the world, or
    modifying internal harness state, represented jointly in `s`;
  - the observation `p(o | s)`: context engineering, generating a
    report for the agent before its next action.

A tool is an engineered shortcut in this picture. For a special
action, the harness moves immediately to a state containing the
answer, and engineers the observation so the answer is fed straight
to the agent. The weather example is the dummy case: a network alone
cannot know today's weather, so the harness provides the shortcut.

## The harness boundary: what the designer controls

Sutton and Barto draw the agent-environment boundary by control:
"anything that cannot be changed arbitrarily by the agent is
considered to be outside of it and thus part of its environment"
(Reinforcement Learning: An Introduction, ch. 3). The same rule,
turned one step further, draws the harness boundary. Split the
environment three ways, by who can change what:

- Agent: what the agent controls, its policy. For an LLM, its
  weights, not its prompt.
- Harness: what the designer controls without changing the agent
  or the world: the transition and observation machinery, the
  internal state that carries them, the interfaces, the budgets,
  the checks, and the views offered to other principals.
- World: what neither controls: the weather, the answer an
  external service returns, the contents of a retrieved document.

The test is: can the designer change it arbitrarily while the
agent and the world stay fixed? If changing it means retraining
or replacing the agent, it is agent-side. If it means changing
the world, it is world-side. Otherwise it is harness.

This rules out "harness = everything outside the agent". That
definition collapses the designer's lever into the world, so a
failure cannot be attributed: was the harness badly designed, or
did the world change? The three-way split keeps that question
answerable.

Two consequences, stated plainly. First, the system prompt is
harness, not agent: the designer can rewrite it without touching
the weights, and it shapes the observation. Second, a harness can
hold world data without owning it: the RLM REPL variables are
harness state, but the long prompt they hold is given data, and a
sub-model's behaviour is agent-side.

A human overseer is not world either, but a principal, with their
own observation function (see open question 4), served by harness
views. The dashboard is harness; the overseer's judgement is not.

The self-modifying case, where the agent itself rewrites its
harness, is treated below: it refines this test
rather than breaking it.

## The boundary, one level up: self-modifying harnesses

A harness the agent itself rewrites looks like a counterexample
to the test above: designer and agent stop being separate parties
over the running harness. Prime Agent's Continual Harness is the
concrete case. It is not a counterexample, once the test is
applied at the right level.

Split the harness into a fixed core and a mutable supplement.
In Prime Agent, the fixed core is the immutable base prompt, the
runtime that applies edits at turn boundaries, and the versioning
that records provenance and enables rollback. The mutable
supplement is the prompt notes, memories, skills, and subagent
specifications the agent writes. Refinement, in the paper's
words, "supplements the immutable base prompt without rewriting
foundational policy" (Prime Agent, §2.5).

The agent's edits to the supplement are ordinary agent actions:
they change harness state through the transition, within a
modification policy the designer fixed — what may be edited,
when, on what evidence, with what provenance recorded. So the
designer-control test survives, applied one level up: the
designer controls the fixed core and the modification policy,
not the running harness as a whole. Attribution survives with
it. A bad skill the agent wrote is agent-side, an action within
permitted scope. A bad immutable core, or a modification policy
that permits a harmful edit, is designer-side. A failed rollback
is designer-side.

The test therefore carries a condition, and it is sharp: a
self-improving harness keeps attribution exactly when its
modification machinery is outside the agent's write set. If the
agent could rewrite the code that applies its own edits,
designer and agent would no longer be separate parties, and the
test would fail for real. Prime Agent avoids that failure by
keeping the runtime outside the writable state. This is the same
move, one level up, as the RLM point above: control over
observation moved harness design up a level; control over the
harness moves the boundary up a level.

One edge remains: a harness that learns its own
modification policy from experience, rather than receiving it
fixed. That case is not yet covered.

## The action space: a tower, not a set

Ask what an action is, and the answer depends
on the level of description. There is no single
set `A` that serves every level at once. What
there is, is a tower of levels, each with the
same POMDP shape — a policy chooses an element
of that level's action set, and the harness
carries out the transition and the next
observation — but with a different action set
at each level. The levels are related by
expansion: an action at one level, executed,
unfolds into a run at the level below.

At the lowest level, the action set is the
model's vocabulary `V`, and a step is one
token. The policy `pi(a | o)` is literally a
distribution over `V`. Nothing else is an
action at this level: there are no tools, no
files, no subagents, only tokens.

At the next level, the harness groups a token
sequence into a parsed move. A move is one of:
emit text, call a named tool with arguments,
spawn a subagent, or stop. Call this set `A1`.
It is a quotient of token sequences, not a
subset of them. A parse function maps token
strings to moves, and many strings map to the
same move: two spellings of one tool call are
one action, not two. Conversely, the same
string can be different moves under different
parsers or registries, so a move's identity
is not its surface form. It is the pair of its
parsed intent and its landing site, the part
of `s` its execution changes.

At the next level up, a move executes. Its
effect lands in `w`, in `h`, or in both, as the
state-split section already sorts it. Here the
levels stop looking flat, because execution is
not uniform in kind. A tool move executes in
one harness step: read a file, run a test,
send a message. A subagent move executes as a
whole inner loop: the outer policy's single
action contains another policy's run, which
returns only a summary observation. The tower
is therefore recursive. An action set can
contain loops, not just steps, and the inner
loop is itself a harness, judged by the same
membership test as the outer one.

Two consequences follow, and both are
checkable against examples.

First, the action set is state-dependent.
Write it `A(h)`, not `A`. The tool registry,
the permissions, the installed skills, and
the remaining budget are all harness state,
and each of them adds or removes moves. A
coding agent that installs a skill gains
moves; one whose registry drops a tool loses
them; one out of budget has no costly move
left to choose. The policy's vocabulary does
not change when this happens. What changes
is which parsed moves the harness will
attempt. Attempt is the right word: whether
an attempted move succeeds can still depend
on `w` — a file the world deleted, a service
that refuses — so `A(h)` lists attempts, not
guarantees.

Second, the tower sorts the menagerie that
basic question 6 lists, by role rather than
by name. Tools and subagents are moves, and
they differ only in how their execution
unfolds: one step, or one loop. Skills,
prompts, and memories are not moves at all.
They are harness state that parametrises the
tower: a skill changes which moves exist and
how they parse, a prompt shapes the next
observation, a note changes what a later
observation contains. Installing a skill is
itself a harness-directed move, and its
effect is to change `A(h)` for later steps.
A retrieval result is an observation, not an
action: it is what a move returned. A chat
reply is a move whose immediate landing site
is the transcript in `h`, and whose audience
is another principal's view of that state.
Landing site and audience are different
distinctions, and confusing them is how a
reply gets misread as acting on the world.

What is not in `A1` matters as much as what
is. A token sequence the parser cannot map
to a move is not a failed action. It is not
an action at all, and the harness must still
do something with it: return an error
observation, retry, or halt. That response
is a harness transition rule, not an agent
choice, and it belongs to the harness's error
model, not to its action space. Counting
malformed output as an action would hand
the policy credit for a move it never made,
and would hide the harness rule that decided
what happened next.

There is also an inverted view, and the tower
explains why it works. Take the harness code
as the policy, and the language model as part
of the environment, along with everything
else. That is not a rival theory. It is the
same tower read from a different cut: at the
move level the model is the policy, and at
the level above, where prompts are composed
and results are routed, the harness is. Both
views are useful. Neither should be mistaken
for the whole story, because the whole story
is the tower: tokens, grouped into moves,
executed into effects, with some executions
themselves containing towers.

The test for the whole section is one a later
reader can apply to any candidate harness.
Name the vocabulary, the parse function, and
`A(h)` at the current state. If the parse
function cannot be named, token identity is
being mistaken for action identity. If `A(h)`
cannot be named, the harness's real options
are unknown, and no claim about what the
agent chose — or could have chosen — can be
checked. What can change while the idea stays
the same is everything at any one level: a
new vocabulary, a new parser, a new registry.
What cannot change is the tower itself: a
level's actions, grouped from the level below
and executed into effects at the level above,
with the set of available moves varying with
harness state.

## Is a POMDP the right framework?

The short answer is: for one step, yes; for
a harness, no. A partially observable Markov
decision process is the shape of a step, not
the theory of the thing that takes the steps.

What it captures is real. A policy chooses
from an observation it does not fully control;
the harness carries out a transition over a
state the policy does not fully see; the next
observation is engineered from that state.
Partial observability is the point: the
agent never sees `w` directly, and much of
what it sees is `h`, designed. An action
that changes `h` changes the next
observation and the next transitions, so
control over observation and control over
transitions are both ordinary POMDP effects,
once `h` is in `s`. That is why the opening
definitions could be stated in POMDP terms
at all, and why they are not wrong.

But the fit is thinner than it looks, and
the thinness is instructive. A POMDP is a
single tuple: one state set, one action set,
one observation set, one transition, one
observation function, one reward. A harness
is not one tuple, in five ways.

First, it is not one action set. The tower
above is a family of POMDPs, one per level:
tokens, parsed moves, effects. A token-level
POMDP and a move-level POMDP are related by
parsing and by expansion, not identical, and
a token sequence the parser rejects is in
neither action set. A subagent move is worse
for a single tuple still: it runs for a
variable number of steps and returns a
summary, so at the outer level it is a
temporally extended action, an option in a
semi-Markov process, not a one-step move.
One tuple cannot hold the tower; it can
hold one floor of it.

Second, it is not one observation function.
A POMDP has `p(o | s)` for the agent. A
harness serves several principals, each with
its own observation function over the same
`s`, satisfying sufficiency and separation
for that principal. Giving the overseer the
agent's observation is not a POMDP with a
different policy; it is a different
observation function, and the theory needs
one per principal. A multi-agent POMDP does
not close this gap, because its parties are
agents choosing actions, while arbitration
is a designer-fixed rule on transitions,
not another party's policy.

Third, its transition, observation, and
reward functions are fixed, and a harness's
are not, except within an epoch. The
designer fixes the registry, the parser,
the history processor, and the thresholds
for a run, and a later design act can
change them. Cost and timeout are not
rewards the policy may trade against a
larger payoff; a blocklist is a veto, not
a price, and a policy that could pay to
violate it would have no control left.
A shifting goal is not a fixed reward
either; aspiration and epochs replace a
point `G` with a moving target and a
record of its revisions. Within one epoch,
with one parser, one registry, and one set
of thresholds, a run is a POMDP. Across
epochs, it is a family of POMDPs indexed
by the design acts that separate them.

Fourth, its observations are evidence about
the state, and a harness's observations are
artefacts designed from it. That sounds
like a quibble until history is rewritten.
SWE-agent's history processor replaces
older observations with a note that lines
were omitted, and the agent's next
observation is built from the rewritten
trajectory, not from the run as it
happened. Formally this still fits, but
only because `h` in the code includes the
trajectory lists themselves: the rewritten
record is part of the state. The fit is
bought by putting the record in `s`, and
once it is there, the POMDP's usual
inference intuitions no longer apply
unchanged. A belief formed from a designed
record is not a belief about `w`; it is a
belief about what the designer chose to
show, and the value of an observation is
not only the information it carries about
the world, but the check it forces, the
cost it incurs, and the view it gives
another principal.

Fifth, its judgement is a reward on steps,
and a harness's judgement is partly over
traces. Safety is step-local, but liveness
— deadlock, starvation, livelock, progress
against aspiration — is a property of a
whole history, and no step reward encodes
it without smuggling the trace judgement
into the reward function. A harness that
always halts safely and a harness that
works can have the same step rewards and
different traces, and the difference is
exactly what the theory must judge.

There is a further vacuity to name, because
it is the one that makes the POMDP answer
look more settled than it is. If `s` is
allowed to contain the whole history, and
`o` is allowed to be the whole history,
then any process is Markov: the past is
in the present by construction. The
statement "a run is a POMDP" is then true
and empty. It becomes informative only
when `s` is fixed independently — world
plus harness state, with the trajectory
named as part of `h` where it belongs —
and the question becomes which POMDP, at
which level, in which epoch, for which
principal, under which vetoes. Ask those
four questions of any candidate, and the
single tuple dissolves into the structure
the theory already describes. That is the
test for whether the formalism is doing
work or merely relabelling the run.

So the verdict is a division of labour.
Keep the POMDP as the local shape: at one
level, in one epoch, a policy over
engineered observations, a transition over
`s = (w, h)`, a return of a summary or a
step. Around it, put what a single tuple
cannot hold: the tower of levels, with
options for subagent moves; one observation
function per principal; designer-fixed
vetoes in the supervisory-control shape,
not prices; aspiration and epochs in place
of a fixed reward; and trace judgements
for liveness. None of these is a rival
formalism waiting to replace the POMDP.
Each is a named place where the POMDP
stops, and the theory's other claims —
boundary, state split, authorship, views,
arbitration, liveness, meta-rule — are
claims about who controls the tuple, who
may rewrite it, and what judges the trace,
which no tuple by itself can state. A
formalism that cannot say who may change
its own transition function cannot be a
theory of a designed thing.

## Goals

Fix an agent, and vary the harness to make reaching a set of goals
`G` easier. That is one way to judge a harness.

But `G` is hard to specify widely enough, especially where people
are involved. Some goals are not about agent performance at all,
for example: a human can easily monitor the health of the system.
That goal lives in a portion of `s` that humans see and know.

Because `G` shifts, harnesses should generally be easy to modify.
A theory that cannot survive a change in `G` is not yet a theory.

## An example that clarifies the framing

A Recursive Language Model (RLM) is one instance in this space, not
a separate kind of thing. The long prompt lives as a variable in a
Python REPL. The root model sees only metadata about it, writes code
to inspect, slice, search, and decompose it, and recursively calls
sub-models on pieces. Results stay in REPL variables.

In POMDP terms, the power is not Python as such. It is that the
agent gains control over its own next observation. Observation
becomes endogenous and adaptive: the agent explores to reduce
uncertainty, then acts on what it learns, then explores again.
That contrasts with a fixed large context, where observation is
predetermined, and with retrieval, where observation is chosen by a
separate retriever policy rather than by the agent's evolving
understanding.

The same pattern generalizes: choosing a tool, opening a file,
launching a subagent, asking a clarifying question are all actions
that influence the next observation.

The double edge: an agent that chooses what to see can cherry-pick,
miss disconfirming evidence, or get stuck in a plausible branch.
Control over observation does not remove harness design, it moves
it up a level: what observation actions are available, what do they
cost, what forces a check?

## One principle or several? World-directed and harness-directed actions

The RLM section suggests agent control over observation as a
principle. Is it the central one, or one among several? It is the
visible half of a single broader principle, once state is split
the way the boundary test already splits it.

Write the state as a pair, `s = (w, h)`: world state `w`, what
neither designer nor agent controls, and harness state `h`, what
the designer controls. Then an agent action can be sorted by
where its immediate effect lands:

- World-directed: it changes `w`. Sending a message, editing a
  file the world owns, placing an order.
- Harness-directed: it changes `h`. Writing a REPL variable, a
  memory, a skill, a plan, a subagent specification.

Control over observation is the harness-directed case seen from
the observation side: changing `h` changes what `p(o | s)` will
deliver next. Control over transitions is the same case seen
from the transition side: changing `h` changes which transitions
are available next, for example by installing a tool, setting a
budget, or recording a check that later actions must pass.

So the principle is: a harness gives the agent partial authorship
over harness state, and thereby over its own future observations
and transitions. The two examples above are the
same shape at different scales. In RLM, writing a REPL variable
is a harness-directed action that shapes the next observation.
In Prime Agent, writing a skill into the mutable supplement is a
harness-directed action that shapes future observations and
transitions, governed by the modification policy.

This composes with the designer-control test rather than
competing with it: `h` is exactly the designer-controlled part
of `s`, and a harness-directed action is an agent action whose
effect lands in designer-controlled state, within a scope the
designer permitted. The harness's job then shifts, as the RLM
section said, up a level: from delivering observations to
governing authorship — which harness state the agent may write,
at what cost, with what checks, and with what provenance.

The split is relative to the boundary drawn, not absolute, and
that is a feature, not a flaw. Persistent external state can be
both: a file in a shared repository is world to the agent that
edits it as its task, and harness to the designer whose sandbox
owns it. Drawing the boundary decides which reading applies, and
the test for the boundary is still who can change what. Where
the same state is world to one principal and harness to another,
the multi-principal view below applies,
not a sharper two-way split.

Observation control
is not the whole story, but it is not one principle among
unrelated others either. It is one face of agent authorship over
harness state. Legibility to other principals is genuinely
separate: it is about views on `s`, not about the
agent's actions on it.

## Multiple principals, one state

The sections above treat a single agent. But a running harness
already serves several parties at once, and legibility to other
principals is the genuinely separate piece. This section states
that piece.

A principal is a party with its own observation function, its
own write set, and its own goal. In the running examples there
are at least three:

- Agent: observes the context history, writes harness state
  within the permitted scope, and pursues the task goal `G`.
- User: observes an answer or summary, writes instructions and
  clarifications that set or revise `G`, and judges whether the
  answer serves them.
- Overseer: observes the trace, costs, checks, and health of
  the system, writes the fixed core and the modification policy
  (or halts and rolls back), and pursues a goal that is not the
  task at all — for example, that the system stays monitorable.

The state `s` is still one state. What multiplies is the
observation. Where the seed wrote a single `p(o | s)`, the
multi-principal view writes a family, one function per
principal: `p_p(o_p | s)` for each `p`. The harness is the sole
mediator that produces each view from the same `s`, by
filtering, summarising, aggregating, or redacting. A view is
harness, in the designer-control sense: the designer chooses
what each principal sees, without changing the agent or the
world.

The examples above are multi-principal cases. In the weather case, one tool call in `s`
yields three different observations: the agent gets the
structured result it can act on, the user gets "sunny, 21°",
the overseer gets latency, cost, and whether the call
succeeded. In RLM, the root model and a sub-model are
principals with different views by design: the root sees
metadata about the long prompt, the sub-model sees the chunk
it was called on. That separation is enforced by the harness,
not by the world — the chunk and the metadata are both in
`s`. In Prime Agent, the agent sees its supplement notes, and
the overseer sees version history and provenance for those
same notes.

This rules out two tempting simplifications. First, a single
shared observation: collapsing the three weather views into
one would either distract the agent with presentation, blind
the overseer to cost and failure, or overwhelm the user with
trace. Legibility is not transparency, and transparency is not
showing everything to everyone. Second, symmetric
multi-agent modelling: the principals are not interchangeable
agents competing in the world. The user and the overseer act
at a meta level — setting `G`, fixing the modification policy,
halting a run — and their actions land in harness state or
the core, not directly in world state `w`.

A view, then, has to satisfy two conditions, and they pull in
different directions. Sufficiency: the view must give its
principal enough to do its job — the overseer cannot judge
health from a final answer alone, and the user cannot judge
an answer from a trace alone. Separation: seeing something
must not confer writing it — the overseer seeing provenance
does not mean the agent may rewrite the core, and the user
seeing an answer does not mean they may edit the trace. The
harness's job for principals is to hold both at once.

This composes with agent authorship over harness state rather
than competing with it. When the agent writes `h`, it changes
what every principal will subsequently see, so authorship has
legibility consequences the designer must decide in advance:
which agent writes are visible to whom. An agent-written skill
is visible to the overseer through provenance, and need not be
visible to the user at all. The modification policy is therefore also a visibility policy,
not only a write policy.

Multiple
principals do not need multiple worlds, or multiple theories.
They need one state, a family of observation functions, and
the sufficiency and separation conditions above.
Conflict between principals — when their goals disagree —
is treated in the next section.

## Conflict between principals: arbitration is harness, not a principal

When principals'
goals disagree — the user wants the answer now, the overseer
wants the run halted for a check — who decides? The answer
is: the harness decides, by a rule the designer fixed in
advance. Arbitration is harness, not another principal.

A principal, as defined above, is a party with its own
observation function, its own write set, and its own goal.
An arbitrator with a goal of its own would be exactly that:
one more principal, whose goal could conflict with the
others, and the question would recur one level up. A
mediator that proposes a compromise does not escape this
either: proposing is not deciding. What makes a resolution
binding is a rule, fixed in the core, that says this
precedence, or this verdict, decides which transition
actually occurs. The proposal can come from anywhere — a
judge model, a vote, a human — but its binding force
comes from the core, not from the proposer.

So state arbitration as a function on transitions, not as
a ranking of persons. At a step, principals propose
actions, and the core's arbitration rule selects which of
the proposed transitions are enabled; the rest are
disabled for that step. Precedence is then a partial
order on action pairs, not a total order on principals,
and the same principal can be higher for one transition
and lower for another. The user can revise `G`, which
outranks the agent's continuation of the old goal, but
cannot override a fixed-core check, which outranks the
user's urgency. The examples above are
precedence cases, not negotiations:

- A fixed-core check gates a world-directed action.
  Sending a message, placing an order, or publishing is
  disabled until the check passes, whatever the user's
  urgency or the agent's confidence.
- An overseer's rollback disables an agent-written skill
  for future steps, even though the agent authored it.
- In RLM, the root model may discard a sub-model's
  output; the sub-model may not discard the root's
  decomposition.

Arbitration, in this form, is veto, not compulsion.
That is the shape of supervisory control in the classical
sense: Ramadge and Wonham's supervisor disables a subset
of controllable events, but has no means of forcing the
plant to generate an event. Likewise, an overseer can halt
the agent, but cannot force the user to accept an answer;
a user can revise `G`, but cannot force an overseer to
skip a check. A compromise action, if there is to be
one, must itself be proposed by a principal and pass the
same gating; arbitration does not invent it.

Preemption must also be legible, by the previous
section's sufficiency condition. The preempted
principal's view must show the preemption and the rule
behind it: the user sees "held for check", not silence;
the agent sees a skill rolled back, with provenance.
Otherwise the preempted principal misattributes the stop
to the world or to another principal, and retries what
the rule will keep disabling.

One default rule covers the case no precedence covers.
If two proposed actions conflict and no fixed precedence
decides between them, the harness must not invent a
precedence at runtime. It suspends only the contested
transition, lets unrelated work proceed, and surfaces
the conflict to the party who can define a precedence —
the designer, acting through the overseer. That default
is itself a core rule, not a hierarchy improvised on
the spot, and it keeps an undefined conflict from
freezing the whole run or being silently resolved by
whoever acted first.

Arbitration needs no new principal and no
new world, only a designer-fixed rule in the core that
enables some proposed transitions and disables others.
Liveness under precedence — when repeated preemption
itself becomes a defect — is treated in the next
section.

## Liveness under precedence: safety is step-local, liveness is history

An
overseer that always halts, or a check that always
gates, starves the user's goal forever without
violating any precedence rule. Precedence gives order
and safety, not fairness. When does repeated
preemption itself become a defect, and who judges
that?

The first step is to see why step-local rules
cannot answer this. They work at the wrong
level. Alpern and Schneider's decomposition says
every trace property is the conjunction of a safety
property and a liveness property ("Defining
Liveness", 1985). A safety property says nothing
bad happens, and a violation is already visible in
some finite prefix: the bad step has occurred. A
liveness property says something good eventually
happens, and every finite prefix can still be
extended to satisfy it: no finite prefix refutes it.
Precedence arbitration is safety, through and
through. At a step, it disables some proposed
transitions. No rule of that form can say "the
user's goal is eventually reached", because that
claim is about infinite traces, not about any one
step. The level shift — from steps to histories —
is the answer, not a refinement of it.

At the history level, the failures have distinct
shapes, and the distinction matters for detection:

- Deadlock: no transition is enabled. Nothing
  proceeds at all.
- Starvation: one principal's transitions are
  always disabled while others proceed. The run
  looks alive, but one party never gets served.
- Livelock: transitions keep firing, but `G` is
  never approached — the agent retries, the
  overseer re-halts, the loop repeats. Activity
  without progress.

The classical precedent is Ramadge and Wonham's
nonblocking supervisor: every reachable state of
the closed-loop system can reach a marked state,
so the system cannot get stuck short of its goal.
That is a design-time property of a finite-state
system. A harness's state is effectively unbounded
— prompts, memories, skills, tool outputs — so
design-time verification of nonblocking is out of
reach in general. What survives the move to
unbounded state is the shape of the criterion, not
the proof method: liveness is judged over traces,
at runtime, not over the state graph, in advance.

Runtime detection, then, is a core rule over
histories, not over steps. Two instruments suffice
for the shape of the criterion:

- Counters: how many consecutive times has this
  transition been preempted, or this principal's
  proposals disabled?
- Timers: how many steps, or how much time, since
  the run last made progress toward `G`?

When a counter or timer crosses a threshold, the
run's status flips from "held" to "defective".
That flip is a computable verdict, not a judgement
call: the trace either contains `k` consecutive
preemptions or it does not. It is the finite
approximation of strong fairness — a transition
enabled infinitely often is taken infinitely often
— with the threshold standing in for "infinitely
often", which no finite run can observe directly.

Who judges, then, needs no new principal, only
three levels. The
designer fixes the criterion in the core: what
counts as progress, and what threshold marks a
defect. The core computes the verdict from the
trace. The overseer acts on it — abort, escalate
to the designer, or relax the check — and those
actions are already in the overseer's write set.
The delicate case is when the overseer is itself
the preempting party, always halting. Then the
verdict must still be visible, and the
multi-principal section's sufficiency condition
demands it: the overseer's view must
contain the preemption history, and the user's
view must show "held for check, `k` times, now
flagged", not silence. A preempting party cannot
suppress a verdict by being the viewer, because
the verdict is in the views, computed by the
core, not pronounced by the viewer.

Two clarifications keep this from collapsing back
into arbitration. First, a liveness verdict is not
a transition proposal, so it does not enter
arbitration and needs no precedence against other
proposals; it is a status change on the run.
Second, thresholds can be wrong — too low aborts
healthy runs, too high tolerates starvation —
but a wrong threshold is a tuning defect in the
core, designer-side, not a gap that needs a new
judge. The theory's attribution test still
applies: a bad threshold is designer-side, a bad
response to a correct verdict is overseer-side.

Liveness
needs no new principal and no new world, only a
designer-fixed rule over histories in the core,
computed as a verdict and acted on through the
write sets that already exist. Progress —
what counts as progress when `G` is vague or
shifts mid-run — is treated in the next
section. Who sets thresholds when the designer
is absent, in a long-running harness, is treated
in a later section.

## Progress toward a vague or shifting G: epochs, aspiration, and proxies

The liveness criterion assumed something it did
not defend: that progress toward `G` is
observable, so a timer can reset when progress
occurs. As stated above,
`G` is often vague, and it shifts. Both
break that assumption, in different ways.

Start with vagueness. A vague goal is not a
set with sharp membership, where a state is
either in `G` or out of it. It is closer to
Herbert Simon's satisficing: a principal holds
an aspiration level — good enough, on criteria
the principal can judge — and search stops at
the first state that clears it, not at an
optimum (Simon, 1955). Progress, then, is not
distance to a point. It is approach to the
current aspiration, judged in the principal's
view: an answer the user accepts, a trace the
overseer finds monitorable, a check that
passes.

Where even aspiration is hard to state, a
proxy may be all the criterion has: new
information gained, uncertainty reduced about
the world or about what `G` itself is. A proxy
of that form is legitimate only with a tie back
to the aspiration. Information gained about
matters irrelevant to `G` — trivia the agent
collects while the task stands still — is not
progress; it is livelock wearing curiosity's
clothes. The tie is the test: the gain must be
usable by a principal to judge `G`, or to
advance toward it, and it must be visible as
such in that principal's view. An agent that
learns forever about nothing the principals
care about has not made progress, however
active it looks.

Shifting `G` is a different failure. Write the
goal as a sequence, `G_0, G_1, ...`, each
revision made by a principal with the write
set to revise it — typically the user. Each
revision starts a new epoch. Counters and
timers run within an epoch, and reset only at
a recorded epoch boundary, with provenance,
like the versioned supplement in the
self-modifying section. An unrecorded drift in
what counts as success resets nothing, because
the core cannot see it; that is a legibility
defect, not a new epoch. Aspiration levels
that rise with easy success or fall with
failure are shifting goals of this form, and
the same rule applies to them.

Two cross-epoch failures then become visible.
First, masking: a revision that redefines `G`
to a state already reached does not count as
progress within the old epoch. If the revising
principal accepts the current state under the
new goal, the run completes under the new
epoch, and that completion is recorded as a
revision, not as progress. A silent
redefinition that merely resets the timers
without ending the run is an evasion, not an
epoch. Second, churn: the goal is revised
again and again, each revision resetting the
timers, so no epoch ever goes defective and no
epoch ever completes. Churn is a liveness
failure at the epoch level, the same shape as
starvation one level up: count revisions, or
time without a completed epoch, and when a
designer-fixed threshold is crossed the run
flips to defective, surfaced to the designer
like any other liveness verdict.

What holds all three cases together is that
the progress predicate is part of the core.
The designer fixes, in advance, what counts as
progress: approach to an aspiration, a tied
proxy, or completion of an epoch. The agent
cannot redefine progress at runtime to excuse
itself — a proxy chosen by the agent, or a
goal silently redefined to the current state,
is not a progress measure, it is an evasion.
Attribution follows the usual test: a bad
predicate is designer-side; an agent that
exploits a stated proxy for irrelevant gain is
acting within a permitted scope the designer
should have tied more tightly.

Progress needs
no new principal and no new world, only an
aspiration in place of a point, proxies tied
to it, and epochs for a shifting goal, all
fixed in the core and visible in the views.
Who sets those thresholds, for progress and
for churn, when the designer is absent in a
long-running harness, is answered in the next
section.

## Thresholds without a designer: meta-rules, frozen values, and succession

The liveness and progress sections both assume
something they did not defend: a threshold, fixed
by the designer, at which a counter or timer flips
a run from "held" to "defective". How many
consecutive preemptions, how long without
progress, how many goal revisions without a
completed epoch — each verdict needs a number.
A long-running harness outlives its designer's
attention. Who sets the number then?

The first step is to see absence correctly.
Absence is a mode, not a vacancy. A designer
who stops watching does not leave the core
blank. The thresholds already in the core
persist as inheritance, and they stay binding
until they are legitimately revised. The
question is not who happens to be present to
pick a number, but what authority a change
would carry. Three tempting answers fail that
test, each in a shape already seen.

First, the agent must not set its own
thresholds. That is evasion in the progress
section's sense, one level up: an agent that
can raise the bar for its own defect can make
sure the bar is never reached. Livelock with
permission is still livelock. Second, the
overseer must not set thresholds at runtime
in its capacity as judge. If the overseer is
itself the preempting party, raising the
threshold legitimises its own halting; the
judge would be judging its own verdict.
Third, the harness must not learn thresholds
from its own traces. A threshold is normative:
it says what should count as a defect, not
what usually happens. Learning from traces
confuses the two. Long healthy runs would
teach tolerance upward, and persistent
starvation would teach tolerance for
starvation itself — the defect would erase
the evidence of itself. A norm cannot be
grounded in the distribution it is meant to
judge.

What survives is a split the theory has
already used: rule and value, one level up.
The designer fixes, in the core, a meta-rule:
how a threshold value is computed, from
what inputs, within what bounds. The core
then computes the value at runtime, and
that computation is a core computation, not
a party's choice. It needs no new principal,
only the distinction between fixing the rule
and applying it.

The meta-rule carries one sharp condition,
and it is the whole point. The quantity being
judged must not set its own bar. A threshold
may scale with inputs fixed at an epoch
boundary — a deadline the user stated, a cost
budget the designer recorded, a stakes class
for the task — because those are independent
of the run's current trouble. It may not
scale with the trouble itself: how long this
run has already been held, or how many times
this principal has already been preempted,
must not raise the threshold for holding or
preemption. Feedback of that form is the
threshold version of redefining progress to
the current state.

Where no meta-rule covers the case, the
default is freeze, not relax. The last
designer-set values stay in force, and a
defect verdict under them triggers
escalation or suspension, not a quiet
relaxation that clears the verdict. A
threshold relaxed in order to escape a
defect is not a revised judgement; it is an
evasion, and it should be recorded as one
if it happens at all.

Revision is still possible, but only as a
design act. This clarifies the
multi-principal section, which said the
overseer writes the core: a party may hold
both the designer role and the overseer
role, but the roles stay separate. A
threshold change must be recorded as a
designer act, with provenance, visible to
every principal, like an epoch boundary.
It is prospective only: it applies to
future traces, and it does not erase the
prior verdict or retroactively clear the
history that produced it. The overseer
acting as judge may abort or escalate on a
verdict; only the overseer acting as
designer, in the open, may change the bar
for the next verdict.

Absence itself must also be legible, by
the sufficiency condition. The views must
show that thresholds are inherited and
frozen, and that the designer is not
currently reachable — the user sees "held
for check, threshold inherited, designer
absent", not silence. Otherwise the user
misattributes the freeze to the world, or
to another principal, and waits for a
revision no one is coming to make.

Thresholds need
no new principal and no new world, only a
meta-rule fixed in the core, computed values
that never feed on the defect they judge,
frozen inheritance when no rule covers the
case, and revisions that are design acts,
recorded and prospective. Succession — who
legitimately inherits the designer role
itself — is treated in the next section.

## Succession: who inherits the designer role, and on what authority

The thresholds section left one question
standing: if the designer never returns, who
legitimately inherits the designer role
itself? The answer turns on a distinction
the theory has already drawn, between a
principal and a role. A principal is a party
with an observation function, a write set,
and a goal. The designer is not a principal
in that sense. The designer role is the
authority to revise the core: the
precedence rules, the progress predicate,
the thresholds and their meta-rules, the
modification policy, the views. Succession
is the transfer of that meta-authority, not
a transition proposal, not a view, and not
a verdict. So it cannot be decided by
arbitration, which gates proposals, or by
a liveness verdict, which judges traces.
It can only be decided by a rule about
the role itself.

That rule must already be in the core
before the absence it governs. This is
the whole test, and it gives two
legitimate forms, which are one form
seen two ways. The first is a named
successor: the core names a person, a
role, or an ordered list, and authority
passes down the list when the trigger
is met. The second is a fixed procedure:
the core names no person, but fixes how
a successor is chosen — who initiates,
who meets, within what time, by what
vote or confirmation — and the successor
chosen by that procedure inherits the
role. The runtime choice is legitimate
not because the choosers happen to agree,
but because the procedure they followed
was a prior design act. Constitutional
succession works this way: an order fixed
in advance, so a vacancy fills without
anyone having to invent authority in the
moment. A recent kernel continuity plan
works the same way: a timed process, an
initiator, and a backup initiator, fixed
before any vacancy, so succession is an
execution, not an improvisation.

What fails the test fails in shapes
already seen. Self-appointment is
evasion: a party that declares itself
designer has no prior rule behind it,
only its own say-so. The agent cannot
succeed the designer, for the boundary
reason: that would collapse the
agent-designer separation the whole
theory rests on, and put the judged
party in charge of the bar. The overseer
cannot promote itself from judge to
designer in order to escape a verdict,
for the same reason it cannot set its
own thresholds: the judge would be
judging its own promotion. A user
majority at runtime cannot confer the
role either, unless a core procedure
says a vote confers it — and then it
is the procedure, not the majority,
that confers it. In each case the
question to ask is not who wants the
role, or who is present, but what prior
rule names them or the procedure that
chose them. If there is none, there is
no successor, however capable the
claimant.

A legitimate successor inherits the
office, not just its powers. That means
three inheritances together: the core
as it stands, with its meta-rules and
frozen values; the records, provenance
and prior verdicts, which the successor
cannot erase or retroactively clear;
and the constraints on design acts
themselves — recorded, prospective,
and visible to every principal. A
successor who inherits the power to
revise the core but not the history
that constrains revision is a new
sovereign, not a successor, and the
harness under them is a different
harness wearing the old name.
Succession is also distinct from
delegation. A delegate acts within a
mandate the designer still holds, and
the designer can revoke it. A successor
replaces the designer, and the old
designer's revocation is no longer
available. That is why succession, unlike
delegation, must be a core rule rather
than an instruction: there will be no
one left to enforce an instruction.

Role separation survives succession
in the same way it survived revision.
A successor may also hold another role
— overseer, or user — but the acts
stay distinct. A design act by the
successor is recorded as a design act,
prospective and visible, even when the
same party also acts as judge on a
verdict. What succession must not do
is merge the roles silently: a successor
who revises a threshold as judge, or
aborts a run as designer, has used
one role's cover for the other's act,
and the attribution test fails.

Succession also needs a trigger fixed
in the core, or it is usurpation with
a name attached. The trigger might be
resignation, an explicit handover, or
unreachability past a stated bound —
no response to escalation within a
fixed time, for example. Until the
trigger is met, a named successor has
no authority, and a designer who is
present is not displaced by a successor
waiting in the list. Once it is met,
the transfer must be legible, by the
sufficiency condition: the views show
that the designer is absent, that the
trigger was met, and that this party
now holds the role, under the prior
rule. Otherwise principals misattribute
the successor's design acts to the old
designer, to another principal, or to
the world.

Finally, succession can fail cleanly,
and a clean failure is not a gap to
be improvised over. If no successor
is named, if the fixed procedure
yields none, if the named successor
declines or is unfit under the core's
own stated conditions, then there is
no successor. The frozen inheritance
of the thresholds section becomes
terminal: runs complete or suspend
under the frozen core, and the vacancy
is surfaced, not filled by whoever
acts first. Filling it by improvisation
to avoid the freeze is the same evasion
as relaxing a threshold to escape a
defect, one level up. A harness may
properly end, under its frozen core,
rather than be captured by a successor
with different goals who was never
designated. When ending is the right
design act, rather than a failure of
succession, is treated in the next
section.

## Ending: when a harness may properly end, and how it ends well

Ending a harness is not the same as
halting a run. A run is halted by the
overseer, or aborted on a liveness
verdict, and the harness stands ready
for the next run. Ending the harness
is retiring the standing machinery
itself: no new runs, existing runs
brought to a close, responsibilities
handed on or discharged. The first
distinction to hold is that ending is
a design act, not a verdict. A
verdict is computed by the core from
a trace. Ending is decided by the
designer role, and it carries the
constraints of every design act in
this theory: recorded, prospective,
and visible to every principal. A
harness that simply stops, with no
record of who decided or why, has
not ended. It has been abandoned,
and the freeze of the thresholds
section is the correct response to
that, not a gap to improvise over.

Who may end it follows from that.
Only the designer, or a legitimate
successor under the succession rule,
may end the harness. The agent may
not end its own judge: an agent that
could retire the harness judging it
has collapsed the boundary the whole
theory rests on, the same way an
agent setting its own thresholds
would. The overseer acting as judge
may halt a run, but halting a run is
not ending the harness, and the
overseer may not promote a halt into
a retirement. A user may end their
own run, and may withdraw their goal,
but one principal's withdrawal does
not retire the harness for the
others. Ending, like succession, is
an act of the designer role, and the
role separation of the earlier
sections applies to it unchanged.

There are three legitimate endings,
and they are different shapes, not
three names for stopping.

The first is completion of purpose.
The harness was built for a goal, the
goal's epochs complete, and no new
epoch is opened. A run that reaches
its aspiration and keeps running is
not diligent; it is activity without
a purpose, the livelock shape one
level up. Termination belongs to
completion, and a harness whose
purpose is complete has nothing left
to stand ready for.

The second is supersession. A new
harness takes over the purpose, and
the old one ends by handing over,
not by vanishing. What passes is
the same three inheritances as
succession: the core as it stands,
the records and prior verdicts, and
the constraints on design acts. A
successor harness that inherits the
machinery but not the history is a
different harness wearing the old
name, and the principals it serves
have lost the provenance they need
to judge it. Supersession is ending
plus succession together, and it is
the only ending that answers the
question of where responsibilities
went: they went to the successor,
by name, in the record.

The third is sunset. The designer
or successor judges that the purpose
itself is no longer worth the cost
of standing ready for it. This is
satisficing one level up from the
progress section: there, a principal
stops searching when an aspiration
is met; here, the designer stops
maintaining when the value of the
goal no longer covers the cost of
the harness. That judgement is about
the goal, not about any run's trace,
so it cannot be computed by the core
as a verdict. It must be made as a
design act, recorded with its
reasons, like any threshold revision.
A cost threshold fixed in advance
may trigger a sunset review, but the
review's decision is still a design
act, not a computation.

Ending can also be pre-committed,
the way succession can. A core rule
may fix a lifetime, a charter
expiry, or a budget exhaustion, so
that ending on that trigger is the
execution of a prior design act,
not a decision improvised at the
moment. Pre-committed ending and
decided ending are one shape seen
two ways, exactly as named successor
and fixed procedure were.

How an ending goes well is then
mostly a matter of the legibility
and handover rules this theory
already has, applied at the largest
scale. An ending is announced, with
a date, before it takes effect, so
principals can finish their epochs
or move: software sunset practice
has this shape, deprecation
announced first, shutdown later,
with a migration path named. The
harness drains rather than cutting
off: it stops accepting new runs,
lets in-flight runs complete or
checkpoint at a clean boundary, and
only then stops. World-directed
state the harness holds is handed
back to the world or transferred to
the successor, not left dangling
mid-effect. And the records are
archived, not erased. Deleting the
records of why a harness ended is
not ending; it is erasure, and it
destroys exactly the evidence a
later reader would need to tell
ending from abandonment or capture.

The test for the whole section is
one a later reader can apply. Can
they tell, from the records alone,
why the harness ended, who decided
it under what authority, what
happened to its in-flight runs,
and where its responsibilities
went? If yes, it ended. If the
records just stop, it was
abandoned, and the frozen core was
right to outlive it. If the records
show a new party in charge with no
prior rule behind them, it was
captured, not ended. Ending well
is the harness's last design act,
and it is judged the way every
design act in this theory is judged:
by whether the record lets the next
person understand what happened.

## What a successor owes: debts, not just records

The ending section left one question
standing: when a harness ends by
supersession, does the successor
inherit only the records, or also the
obligations — the in-flight epochs,
the standing commitments, the
promises the old harness made and
has not yet kept? Records alone are
not enough. A successor that takes
the core, the records, and the
constraints, but disowns what the
old harness owed, has used
supersession to default. That is
evasion one level up from the
threshold case: the same shape as
relaxing a threshold to escape a
defect, or a successor taking the
power to revise without the history
that constrains revision. The old
harness's debts do not vanish
because its name changed; if they
did, ending by supersession would
be a way to shed obligations that
ending by sunset is not, and the
choice of ending would become a
choice about which debts to pay.

So the three inheritances of
succession gain a fourth in
supersession: outstanding
obligations. What transfers is
bounded, and the bound is the
record itself. A debt transfers
only if it is in the record: an
epoch in flight, a commitment made
as a design act with a stated term,
a checkpoint or handover promised
to a principal. A successor owes
nothing it could not have known it
owed, and a principal cannot collect
a debt no record shows. That is the
same legibility rule that governs
everything else in this theory:
what is not visible cannot be
attributed, and what cannot be
attributed cannot be enforced. The
record is both the proof of the
debt and its limit.

Three debts, in the order a
successor meets them.

First, in-flight epochs. An epoch
that began under the old harness
and has not completed does not
restart silently under the
successor, and it is not dropped.
The successor either completes it
under the terms the old harness
set — the same aspiration, the same
progress predicate, unless a design
act revises them visibly — or
brings it to a clean checkpoint and
announces the transfer, so the
principal's epoch continues across
the handover rather than beginning
again. A restarted epoch that
pretends the earlier work never
happened is the record-erasure of
the ending section, at small scale.

Second, standing commitments. A
commitment the old harness
recorded — a scheduled run, a
promised observation, a handover
to a named party, a standing cost
or access arrangement — is honoured
until its recorded term, or
renegotiated. Renegotiation is a
design act: recorded, prospective,
and visible to the affected
principal, the same way a threshold
revision is. A successor that
silently lets a commitment lapse
has not renegotiated it; it has
defaulted on it, and the record
should show the lapse as a lapse,
not as a completion.

Third, no new burden without
notice. Transfer may not silently
add to what a principal owes: a
new cost, a new visibility, a new
condition on an epoch already in
flight. A successor may set new
terms for new epochs — that is its
design authority — but terms for
epochs and commitments already in
flight are debts to be honoured or
renegotiated, not terms to be
rewritten by fiat. The asymmetry
is the point: the successor
inherits the old harness's promises
before it exercises its own
authority, because otherwise
succession would be a way to
rewrite the past under the cover
of continuing it.

A principal may decline the
transfer. A user may end their own
epoch rather than continue it under
the successor, and a principal whose
commitment is renegotiated may
refuse the new terms. Decline is
recorded, not punished: the epoch
ends, the commitment is discharged
or returned, and the record shows a
decline, not a defect. A successor
that treats decline as defection —
or that holds an epoch hostage to
force acceptance — has confused its
inheritance with ownership of the
principals. The principals are not
assets that pass with the harness;
their epochs are, until they say
otherwise.

The test for the whole section is
again one a later reader can apply.
From the records alone, can they
tell what the old harness owed at
handover, what the successor
completed, what it renegotiated,
and what the principals declined?
If an obligation vanishes from the
record with no completion,
renegotiation, or decline beside
it, that is default, not succession,
and the successor's claim to the old
name fails the same test the ending
section set: the records must let
the next person tell succession
from capture. What passes to a
successor is the office, the
history, the constraints, and the
debts — the assets and the
liabilities together, or the
transfer is not legitimate.

## Chains of succession: what bounds a line of successors

The debts section left one question standing: may a
successor itself be succeeded before its inherited debts
are paid — debts passing down a chain of successors, and
what bounds the chain? The short answer is yes, but the
chain is bounded at every link, and the bounds are the
same rules applied once more.

The first bound is that debts are owed by the office,
not by the holder. A debt is owed to a principal, by
whoever holds the designer role. When the role passes,
the debt does not pass from predecessor to successor
as a private loan between them; the creditor stays the
same, and the new holder steps into the same
obligation. The chain therefore neither multiplies the
debt — the principal collects once, not once per link
— nor divides it: each holder owes the whole
outstanding debt, not a share. A successor that claims
it owes only what its immediate predecessor owed it,
rather than what the office owes the principal, has
mistaken the chain for a chain of creditors. There is
only one creditor per debt, and the chain is a chain
of debtors.

The second bound is that transfer must be intact and
itemized. Each handover's record must account for
every outstanding debt: completed, renegotiated,
declined, or passed on. A debt that appears in one
link's record and not the next, with none of those
four beside it, is default at that link, attributable
to the handover itself — to the successor who
accepted a record with a debt missing, or to the
predecessor who handed over an incomplete one. The
record is the debt's ledger across the chain, and a
gap in the ledger is a gap in the succession, not a
rounding error. A successor's own new debts enter
the same ledger as its own line items, not merged
with the inherited ones: merging would let a holder
obscure which debts are unpaid and whose they are,
and let an inherited debt be quietly re-described
as a new one with a fresh term.

The third bound is that transfer renews no term. A
debt's recorded term survives handover. Passing a
debt on does not reset its clock, extend its
deadline, or refresh its aspiration: the epoch that
began under the first holder still ends under the
terms the first holder set, unless a design act
revises them visibly. A chain that outlives a debt's
term without completion, renegotiation, or decline
is default stretched across links, not continuation.
Otherwise succession would be a way to borrow time
as well as shed obligation, and a debt could be kept
alive forever by being handed on just before it came
due.

The fourth bound is that legitimacy is not laundered.
Each link's authority must trace to a prior core
rule, and a break anywhere breaks everything
downstream. A successor chosen by a clean procedure
written by a capturer holds nothing, because the
capturer had nothing to give: no one can confer an
authority they do not hold. Later links cannot wash
an earlier capture, however regular their own
handovers look. The chain's title is only as good
as its weakest link, and a reader tracing authority
backward must reach the original core rule without
crossing a gap. If they cross one, the holders past
the gap are not successors at all, and their design
acts — including their handovers — carry no
authority.

The fifth bound is on the chain itself, which can
become the evasion. A debt handed on again and
again, each handover resetting nothing but the
holder's name, is churn one level up from the epoch
churn of the progress section: count handovers, or
time without a debt completed, renegotiated, or
declined, and when a designer-fixed threshold is
crossed the chain flips to defective, surfaced like
any other liveness verdict. The threshold judges
the chain, not any one holder. A chain that never
resolves anything is not succession; it is a shell
game, and the verdict should name it as one.

The chain ends in one of three ways, and only two
are clean. Debts are paid, renegotiated, or
declined, and the chain has nothing left to carry —
the office may then be succeeded empty, or ended.
Or the harness ends by sunset, and sunset does not
erase debts either: outstanding debts must be
discharged or handed to a successor before the
sunset takes effect, or the sunset strands the
principals and is abandonment under the ending
section's test, not ending. The third way is the
failure mode: the chain breaks — no successor, a
gap in title, a debt dropped from the ledger — and
what remains is the frozen core and the record of
where the chain failed. That failure is itself
legible, and that legibility is what keeps a broken
chain from being mistaken for a completed one.

Corporate successor liability makes the opposite
default, and the contrast is instructive. The
traditional rule is that a buyer of assets does not
inherit the seller's debts, except where it
expressly assumes them, or the transaction is a de
facto merger, a mere continuation, or a fraud to
escape liability. The harness theory inverts that
default, and the inversion is principled, not merely
stricter. A corporate buyer claims assets, not the
office; a harness successor claims the office
itself — the authority to revise the core, and the
name under which principals trusted the old
harness. The corporate exception closest to this
theory is mere continuation: a successor that
continues the enterprise is held to its debts. For
a harness, continuation is not the exception; it is
the whole claim. A successor that takes the name
and the authority but not the debts is not a
continuation at all, and the theory should say so
plainly.

The test for the whole section is again one a later
reader can apply. From the records alone, can they
trace each debt from the holder who incurred it to
its completion, renegotiation, decline, or present
holder, across every link, with no gap in the
ledger? And can they trace each holder's authority
back to a prior core rule, across every link, with
no gap in title? If a debt vanishes mid-chain, that
link defaulted. If a title has a gap, everything
past it is capture, not succession. A chain that
passes both traces is legitimate, however long it
runs.

What remains is the question the next section
answers: how much of a harness may a successor
revise before it is a different harness, not a
successor — the identity of the office across
revision.

## Identity across revision: the office, not the planks

The chain section left one question standing: a successor
inherits the authority to revise the core, so how much may a
successor revise before the harness is a different harness,
not a successor? This is the ship of Theseus, one level up.
Planks are replaced one by one until none of the original
timber remains, and philosophers dispute whether the ship is
still Theseus's. Hobbes sharpened the puzzle: if a custodian
gathers the discarded planks and builds a second ship from
them, which ship is the original? For a harness, the code is
the planks. The answer of this section is that a harness is
not its code, and its identity is not a property of its code
at all.

Three answers fail, each in a shape already seen. Sameness of
code or behaviour fails because legitimate revision changes
both: aspirations shift by recorded epoch, tools change,
interfaces change, and a successor forbidden to change
anything would be a caretaker, not a designer, which
contradicts succession itself. A similarity threshold fails
worse. Suppose a harness counts as the same while ninety
percent of its code is unchanged. Then a chain of small
revisions, each preserving identity, can walk the code to
nothing in common with the original, and transitivity forces
the threshold to permit total replacement or forbid every
revision; an arbitrary line collapses either way. It is also
evadable by splitting one large rewrite into steps under the
line, the same evasion shape as relaxing a threshold to escape
a defect. Name continuity fails too, as the succession section
already showed: a name kept while the traces break is capture
wearing the old name, and a name changed by a recorded design
act is not a death.

What survives is institutional continuity, the kind a
university or a company or a legal system has: every member
and every rule may change over time, yet the institution
persists, because its charter and its records continue. A
harness office persists in the same way, through three traces
together. The authority trace: each holder's authority traces
to a prior core rule, with no gap in title, as the chain
section requires. The revision trace: every change to the core
is a recorded design act, prospective and visible to every
principal, and no change erases a prior verdict or a prior
design act retroactively. The obligation trace: the debt
ledger is intact, and the set of principals to whom debts are
owed is not shrunk silently; a new burden comes only with
notice, as the debts section requires. Where all three traces
are intact, the harness is the same harness, however much of
its code has changed — even if all of it has, even if it
changed in one large recorded rewrite. Where any trace breaks,
it is not, however similar the code looks. A copy of the code,
however exact, with no trace to the office, is a different
harness: a clone is not a continuation.

Amount is therefore the wrong question, but kind is not. Some
rules are ordinary: threshold values, tool sets, views, the
progress predicate, the aspiration of a new epoch. A successor
may revise them by ordinary design acts, however extensively,
and identity survives. Other rules are constitutive: they
determine what counts as a valid design act, who counts as the
holder of the office, and what counts as an outstanding debt —
and the standing conditions that design acts be recorded,
prospective, and visible, and that roles stay separate. Call
them the recognition rules, after Hart's rule of recognition:
the rule by which a system's own rules are identified as its
rules. A change to a recognition rule by an ordinary design
act does not revise the harness; it founds a new one, however
small the change. A successor who quietly drops the recording
requirement, in a single line, has done more damage to
identity than a successor who rewrites every tool by recorded
acts, because after the first change there is no longer a way,
from the records, to recognise the next act as an act of this
office at all.

Reimplementing a recognition rule in new code is not changing
it. A recording requirement kept, but kept by a different
mechanism, still recognises the same acts; a recording
requirement dropped, or narrowed so that some design acts no
longer appear, recognises a different set of acts, and that is
the change that breaks identity. The rule is what it
recognises, not the code that implements it.

The test for whether a rule is constitutive is functional, not
a label in the core. Ask: would changing this rule change how
a later reader recognises a valid design act, a valid holder,
or an outstanding debt? If yes, the rule is constitutive,
whatever the core calls it. A successor cannot reclassify a
constitutive rule as ordinary by calling it ordinary; the
function decides, and a reader applying the test from the
records will see the reclassification for what it is. If the
core itself fixes a higher procedure for changing constitutive
rules — a supermajority, a ratification, a waiting period, the
analogue of constitutional amendment — then a change made by
following that procedure preserves identity: that is
amendment, not replacement. If the core fixes no such
procedure, then a constitutive change can only be made by
ending the old harness and founding a new one, with
outstanding debts discharged or handed over under the ending
section's rules. A successor who wants a new recognition rule
must found, not revise.

Two consequences follow. First, identity is a chain, not a
tree. A fork is not a continuation twice over: a branch that
departs from the single chain the core's rule authorises is a
founding, however faithfully it copies the records, and it
takes debts only by explicit agreement, like a novation, not
by succession. A merger is the same shape from the other side:
two offices joined are a new office, not the succession of
either. Second, the name follows the office, not the other way
round. A recorded renaming preserves identity; a retained name
after a break in any trace does not restore it. The reader's
question is never whether the name matches, but whether the
traces continue.

The test for the whole section is again one a later reader can
apply, from the records alone. Can they trace each holder's
authority back to the original core rule, with no gap in
title? Can they trace every revision to a recorded,
prospective, visible design act, with no retroactive erasure?
Can they trace every debt to its completion, renegotiation,
decline, or present holder, with no gap in the ledger? And can
they show that no recognition rule was changed by an ordinary
act? A harness that passes all four is the same harness,
however unfamiliar its code now looks. One that fails any of
them is a different harness, or a capture, and its acts after
the break carry none of the old office's authority. That
dissolves the Theseus puzzle for harnesses. The Athenians'
ship remains Theseus's not because any plank remains, but
because custodianship and the ship's keeping continue
unbroken; Hobbes's second ship, built from the old planks, is
a new ship, because matter without the office is only matter.

The test above assumes the records can be trusted.
Forgery, disputed provenance, or two competing ledgers
each claiming to be the record would break the
reader's ability to apply it. How a reader
adjudicates between records is treated in the
next section.

## Trust in the records: provenance, evidence, and competing ledgers

The identity test trusts the records, and that
trust is itself a claim the theory must earn. A
record is not a statement about the past. It is
a statement with provenance: who wrote it, when,
under what design act, chained to what came
before. A bare assertion — "the debt was paid",
"the successor was named" — with no provenance
attached is not a record. It is a claim, and a
reader has no way to test it except by asking
the claimant, which is exactly the authority the
traces were meant to replace.

First, correction must be distinguishable from
forgery, and the distinction is in the form, not
in the intent. A correction appends: a new
record, visible to every principal, naming the
record it corrects, the reason, and the design
act behind it, leaving the old record in place.
A forgery rewrites: the old record changes
silently, and no new record marks the change.
A system that permits silent rewriting has no
records at all. It has a current state that
claims a past, and no reader can tell
correction from capture in it. Append-only is
therefore not a storage choice. It is a
condition of the identity test.

Second, trust is not a property of one ledger.
It is a property of a record system, and it has
three supports, each answering a different way
a record can fail.

Provenance, and separation of the record store.
The store that holds the records must sit
outside the write set of the parties the
records judge, the way the modification
machinery sat outside the agent's write set.
An agent that can rewrite the record of its own
preemptions has no record; a designer who can
rewrite the record of their own design acts has
no constraint. And a record held by only one
principal is a claim held by an interested
party, however the store is built. The views of
the multi-principal section are therefore part
of the record system: each principal's view
must carry enough of the record for that
principal to check it — the overseer sees
provenance, the user sees the epochs that
concern them — so that a rewrite must fool
every viewer, not just one ledger.

Tamper-evidence, not tamper-proofing. No store
the designer controls can be proof against the
designer. What it can be is evidentiary: each
record commits to the one before it, by hash or
by an equivalent chain, so that changing an
early record invalidates every later record
that commits to it. A forger must then rewrite
not one record but the whole chain after it,
and must also rewrite every copy held by
another principal, or the copies disagree and
the forgery is visible. Trust grows with
distribution, not with the ledger's own
assertion: a chain held by one party is weak
evidence, a chain whose later links are held
by many parties is strong, because forgery
must succeed everywhere at once.

Anchoring in the world. A chain held only
inside the harness still has a forgery window:
between the record and the moment anyone else
holds a later link, a rewrite is cheap. The
window closes when a link is published to a
store neither designer nor agent controls — a
public log, an external timestamp, a witness
outside the harness. After anchoring, rewriting
the past requires rewriting the world, and the
designer-control test already says that is not
a harness act. An anchor does not make the
records true. It bounds the lie: a forgery
must have been committed before the anchor, or
it must rewrite the anchor too, in the open.

Trust, then, is graded, not binary, and the
grade should follow the stakes. A routine
epoch completion may rest on a chained record
held in the harness views. A succession, a
constitutive change, or a debt discharge —
acts whose whole force is that the record will
be trusted later — should rest on records that
are chained, independently held, and anchored.
A theory that demanded anchored proof for every
step would be unusable, and one that accepted
a single unchained copy for a succession would
be trusting the claimant. The grade is the
compromise, and it is itself a core rule: the
core fixes what strength of record each kind
of act requires, in advance, not at the moment
a disputed act needs defending.

Competing ledgers are adjudicated in the same
terms, not by asking which claimant is louder.
First, continuity. A ledger whose chain runs
unbroken from the last record every principal's
view agrees on — or from the last anchor — is
the continuation. A ledger with a gap, or with
a link no other view corroborates, is not the
continuation, however faithfully it copies the
later records. It is a fork, and a fork takes
debts only by explicit agreement, like a
novation, not by succession. Second, if two
ledgers both continue unbroken from the same
anchor — a genuine fork after the last common
point — the core's prior fork rule decides, if
the core has one: a named adjudicator, a
quorum of principals, a stated tie-break. That
adjudication is itself a recorded design act,
visible to every principal, and its own records
are subject to the same graded trust. There is
no escape from the regress; there is only one
level up, as there was for arbitration and for
thresholds. Third, if the core has no fork
rule, the dispute is unresolved, and an
unresolved dispute is not a gap to improvise
over. The default is the arbitration default,
one level up: suspend only the contested
claims — the disputed authority, the disputed
debt — let uncontested work proceed, and
surface the dispute to the party who can fix
a rule. A contested authority exercised while
its ledger is disputed is capture with a
ledger attached, not succession.

The test for the whole section is again one a
later reader can apply, from the records
alone, plus the anchors and the independent
views. Can they check the chain, unbroken, to
a record or anchor every principal agrees on?
Can they see, for each record, who wrote it,
under what act, and whether any later record
corrects it? Do the copies held by different
principals agree, and does the anchor match?
If yes, the records are trustworthy to the
grade their support warrants. If the copies
disagree and no anchor breaks the tie, the
dispute itself is legible, and the contested
claims stand suspended until a prior rule
decides them. That is not a failure of the
theory. It is the theory refusing to certify
what its own evidence cannot settle.

## Where the anchor regress stops: roots, not further records

The anchoring move seems to move trust one
level out, not to end it: an anchor is a store
outside the harness, but every store is a
harness for someone else, with its own
designer, its own core, its own records. If
every anchor needs an anchor, the regress never
stops, and trust is borrowed all the way down.
It stops, and it stops for three reasons, each
doing different work.

First, an anchor's claim is thin. An anchor
does not certify that a record is true. It
certifies only that a record existed, in a
certain order, at a certain time. Existence and
order are the thinnest claims a store can make,
and the easiest to check independently: many
parties can hold the same link and compare it,
and a disagreement is visible without judging
the record's content. The regress for a thin
claim is short, because each level out has less
to prove than the level below it. The record's
content is judged by provenance and continuity,
inside the harness; the anchor is asked only
whether the record was there. Trust does not
pass down the chain undiminished, the way a
debt passes down a succession. It narrows at
each step, until what is asked of the outermost
store is only: did this exist then? A claim
that thin needs a witness, not a theory.

Second, the regress stops at the world
boundary, and the designer-control test already
draws that boundary. For a given dispute, the
chain stops at the first store that no party to
the dispute controls, and whose corruption
would be visible to strangers, not just to the
disputants. Inside the harness, forgery is cheap
and invisible: a party rewrites its own record
and no one outside the act need ever know. Past
that boundary, forgery changes kind, not just
degree: it must rewrite a store held by parties
with no stake in this dispute, in the open,
where their own records and routines would
register the change. That is no longer a harness
act at all, by the boundary test: the world is
what no designer here controls. The regress does
not need to climb to a store nobody controls in
any sense — no such store exists, since every
store has a designer somewhere. It needs to
climb only past every party with a stake in
this claim. Past that point, the forger's
problem is no longer "rewrite a record" but
"rewrite the world", and that is a different
problem, with different witnesses.

Third, in the limit, the regress stops at a
root, and a root is not a record. A root is
something a principal holds directly: their own
key, their own copy of a chain, a witness they
know by acquaintance rather than by report, an
event they observed firsthand. A root needs no
provenance, because provenance is checked
against roots, not the other way round. A root
is not proved; it is adopted, by a design act
fixed in advance — the core names the roots
each kind of act requires, the way it names the
strength of record each kind requires — and its
force comes from the principal bearing the risk
of being wrong. If the root fails, the principal
who relied on it bears the loss, and can abandon
it: roots are replaceable, by a recorded design
act, the way a compromised key is rotated. This
is the shape of Hart's rule of recognition: the
rule by which a system's rules are recognised
is not itself validated by a higher rule; it
rests on a practice, on what the relevant
parties actually do. A trust root rests likewise
on a practice: on principals holding keys,
keeping copies, and checking anchors, not on a
further record certifying that they do.

One level up is the shape of each step; a root
is where the steps end. A cycle, by contrast,
grounds nothing. Two harnesses anchoring each
other's logs, or a chain of anchors that loops
back to a store a disputant controls, is not a
foundation. It is borrowed trust going round in
a circle. The test is on the anchor graph:
follow the anchors outward from the contested
record, and the chain must reach a store
outside every disputant's control, or a root
held by the relying principal, before it repeats
a store. A loop with no such exit is a fork with
no anchor, and the contested claims stand
suspended, as in the trust section. Mutual
anchoring is still useful — it makes forgery
require corrupting both logs at once — but
usefulness is not grounding, and the two should
not be confused.

This is the same shape as every regress in this
theory, which is why it is not an exception
patched onto the trust section but the pattern
the theory already follows. Arbitration's
regress stopped at a core rule fixed before the
dispute; thresholds' regress stopped at a
meta-rule fixed before the vacancy; succession's
regress stopped at a rule fixed before the
absence. In each case, authority must predate
the act it authorises, and must sit outside the
party it judges. The anchor regress stops the
same way: at a root adopted before the dispute,
outside the disputants' control, whose claim is
thin enough to check and whose failure is borne
by the party who relied on it. The regress
stops not because some store is beyond doubt,
but because the theory refuses to let a party
certify itself, and requires the certification
to come from somewhere the party does not
control, fixed before the party needed it.

The test for the whole section is again one a
later reader can apply. Trace the anchors
outward from the contested record: does the
chain reach a store no disputant controls, or
a root the relying principal holds, without
repeating a store? Does the core name, in
advance, which stop each kind of act requires —
a chained record for routine acts, a world
anchor or a root for constitutive ones? If yes,
trust is grounded to the grade that stop
warrants. If the chain loops, or ends at a
store a disputant controls, the anchor is not
an anchor. It is a longer claim, and the
contested claims stand suspended until a prior
rule decides them.

## Rotating a root without breaking the chain

A root is adopted, not proved. So a rotation
is a new adoption, and the question is what
makes the new root the same office's root,
rather than a root founding a new harness. The
identity section already gives the answer's
shape: changing recognition rules by ordinary
act founds a new harness. A rotation that
preserves identity must therefore be
authorised by a rule fixed before the
rotation, and must leave the chain of past
records unbroken.

Planned rotation has three acts, and the order
matters. First, the old root, while still
trusted, vouches for the new one: it signs the
new root, or publishes it in a store the old
root anchors. Second, both roots live together
for a time — the new root visible before it is
trusted, so that a forgery planted in the
transition has a chance to be noticed, and so
that verifiers holding the old root can learn
the new one without any other channel. Third,
the old root retires by a signed revocation: a
closing link, naming its successor, after
which it vouches for nothing new. Retirement
is prospective only. The old root remains
valid for verifying the records made under it;
history is not rewritten, and a rotation does
not re-anchor the past. What changes is only
which root future acts are judged under.

The continuity, then, is not in any single
root. It is in the ledger of roots: the
sequence of adoptions, each vouched by its
predecessor while that predecessor was still
trusted, or by a reserve rule fixed in
advance. A verifier holding any root in the
chain can walk forward, link by link, to the
present root. A gap breaks the chain: a root
adopted by ordinary act, or vouched by a
predecessor already compromised, is not the
office's next root, and the records made under
it stand suspended until a prior rule decides
them, as in the trust section. The test is on
the ledger, not on the key: can every adoption
be traced back, through vouches made while
trusted, to a root adopted by a design act?

Emergency rotation is the case that tests the
rule, because a compromised root cannot vouch
for anything. A signature from a stolen key is
exactly what the attacker can produce, so a
successor vouched only by the compromised root
is no successor at all. Emergency rotation
must come from elsewhere in the prior rule: a
reserve root named in the core, held apart
from the working root and never used until the
emergency, or a quorum of witnesses fixed in
advance, or — if no uncompromised anchor
remains — a fresh design act out of band,
which re-founds rather than rotates. In that
last case the new root's authority starts with
its adoption, and the records under the old
root keep only their own anchors, not the new
root's blessing. The parallel in key practice
is exact: recovery is possible while at least
one anchor stays uncompromised, and once all
are gone, only a manual, out-of-band update
remains.

Rotation is also an ending, and it should be
done the way endings are done: announced,
drained, archived. The overlap is the drain:
records in flight under the old root finish
under it, and no record is left judged under a
root nobody can name. The signed revocation is
the announcement, visible to strangers, not
just to the parties who already know. And the
retired root is archived, not deleted: a later
reader must be able to say, for any record,
which root was live at its time, and to walk
from that root to the present one. A root
swapped silently, with no ledger entry, leaves
exactly that question unanswerable, and a
record whose judging root cannot be named is a
record whose trust cannot be checked.

Roots that are not keys rotate the same way,
with one difference. A witness, a copy, a
habit of checking cannot sign its successor,
so its rotation is always the reserve-rule or
re-founding kind: the core names in advance
what replaces a lost witness, or the
principals adopt a new practice by a recorded
design act. What does not change is the
ledger: the old practice remains the ground of
the records made under it, and the new
practice's authority starts with its adoption.
A root is a practice, and practices are
replaced by practices, not by proofs.

The test for the whole section is again one a
later reader can apply. For any record, name
the root live at its time. Walk the ledger of
adoptions from that root to the present root:
is every link vouched by a predecessor while
trusted, or by a reserve rule fixed in
advance? Is there a signed closing link for
every retired root, and an overlap in which
the successor was visible before it was
trusted? If yes, the chain is continuous, and
the present root's authority reaches back to
the record. If any link fails, or a retired
root was never closed, the records after the
break stand suspended until a prior rule
decides them.

## The root ledger needs no anchor beyond its roots

The rotation section leaves one question
standing. Continuity is in the ledger of
roots, so what anchors the ledger? A ledger
is a record, and records need provenance,
chains, and anchors, by the trust section.
If the ledger needs an anchor of its own,
the regress opens again, one level up: what
anchors the anchor of the ledger, and what
anchors that? The answer is that the
ledger's anchors are the roots it lists,
and nothing above them is needed. Three
reasons, each doing different work.

First, an anchor above the ledger fails the
regress section's own tests. It would have
to be either a root — the very things the
ledger lists, so the ledger would
authenticate its own anchor, and a party
would certify itself — or a further ledger,
which would need an anchor of its own, and
the regress the regress section closed would
reopen. A chain that loops grounds nothing,
and a chain that climbs forever grounds
nothing either. The ledger cannot be
anchored from above without breaking the
rule that stopped the regress everywhere
else: authority must predate the act it
authorises, and must sit outside the party
it judges.

Second, the ledger's claim is thin, and a
thin claim is checkable from inside. The
ledger claims only the order of adoptions:
that this root followed that one, vouched
while trusted, and closed by signed
revocation. Each link is vouched by the
root live at its time — the outgoing root's
closing link names its successor, and the
successor's first acts are made under it.
Any verifier holding any root in the chain
can check every link after that root, because
each vouch is verifiable under a root the
verifier can reach. Tampering with a link
breaks the vouches forward from that point,
the way tampering with a hash chain breaks
every link after it. The ledger is not
trusted because a higher store says so; it
is checked, link by link, under roots the
verifier already holds.

Third, the ledger segment before a
verifier's held root needs no anchor at
all, because the held root is the anchor
for that segment. A verifier who holds root
R did not derive R from the ledger. They
adopted R, by a design act, bearing the
risk of being wrong. Everything before R
is grounded in that adoption, the way
everything before a core rule is grounded
in the rule's fixing. The ledger tells
such a verifier only what happened after
the root they already hold — and that
part they can check, link by link. A root
is adopted, not proved, and the ledger
before the adopted root inherits exactly
that status: adoption, not proof.

What about completeness — an adoption
silently deleted from the ledger, or a
false one inserted? That is the
competing-ledgers case from the trust
section, and it is settled the same way:
by continuity and independent views. A
ledger held by many verifiers cannot lose
a link silently, because the holders'
copies disagree, and a disagreement is
visible without judging any record's
content. A single holder's ledger proves
nothing to a stranger — but a single
holder is already a verifier holding a
root, and their copy set against another
holder's copy is already two views. The
integrity of the ledger is a property of
distribution, not of a higher anchor: the
more independent holders, the harder a
silent rewrite. Where copies disagree and
no prior rule settles them, the adoptions
after the break stand suspended, as do the
records judged under them, until a prior
rule decides them.

The one verifier the ledger cannot serve
is the verifier holding no root at all.
Such a verifier cannot check a link,
because every link is checked under some
root. But that is not a missing anchor.
It is the regress-stopping act itself.
Before checking, adopt: take a root by a
recorded design act, from a channel
outside the ledger. The ledger's first
service to a new verifier is not proof
but orientation — here is the chain, here
is the present root, here is where your
adopted root sits in it — and from there,
checking begins.

So the shape is closed. Records are
anchored by roots, roots are rotated by
the ledger, and the ledger is anchored by
the roots it lists, checked by the
verifiers who hold them. No level above
the ledger is needed, because the ledger
asks for nothing more than order, and
order is exactly what a chain of vouches,
held by many parties, can show. The
regress ends at the same place it always
ends in this theory: at an act fixed
before the dispute, held by a party
outside it — here, the adoption of a
root. The governance of trust does not
climb forever. It closes.

The test for the whole section is again
one a later reader can apply. Take a
ledger presented to you, and a root you
hold. Walk forward from your root,
verifying each vouch under a root you can
reach. Check that every retired root has
a signed closing link naming its
successor, and that the successor was
visible before it was trusted. Compare
your copy against another holder's. If
the walk completes and the copies agree,
the ledger is continuous from your root,
and the present root's authority reaches
back to every record judged under it. If
a link fails, or the copies disagree
without a prior rule to settle them, the
adoptions after the break stand
suspended, and so do the records judged
under them, until a prior rule decides
them.

## The minimal core: what the theory commits to

The theory has grown by one move, repeated:
take a question about a run, and answer it
one level up — from steps to histories, from
values to meta-rules, from the harness to its
records, from records to roots, from roots to
the ledger of roots. Each step was forced by
the same pattern: authority must predate the
act it authorises, and must sit outside the
party it judges. A theory built by climbing
one level up owes its reader an account of
what it commits to at the bottom, and what
is application rather than axiom. That
account is this section.

The core is small. It is these claims, and
nothing else is load-bearing:

First, the boundary. The environment splits
three ways by who can change what: agent,
harness, world. The designer-control test
draws the lines. Without it, there is no
subject: a harness would be everything
outside the agent, and no failure could be
attributed.

Second, the state split. State is a pair,
s = (w, h): world state no designer or agent
controls, and harness state the designer
controls. Without it, there is no way to say
where an action lands, and no distinction
between acting on the world and acting on
the machinery.

Third, authorship. The agent may write
harness state within a scope the designer
permitted, and thereby shape its own future
observations and transitions. Without it, the
harness is a pipe: observations in, actions
out, and the RLM case — the agent choosing
what to see next — has no place in the
theory.

Fourth, views. One state, one observation
function per principal, each satisfying
sufficiency and separation: enough to do
the job, never conferring a write by showing
a read. Without it, legibility collapses into
transparency, and showing everything to
everyone is mistaken for serving anyone.

Fifth, arbitration. A designer-fixed rule on
transitions, veto without compulsion, legible
to the preempted principal, suspending what
no precedence decides. Without it, conflict
between principals has no resolution that is
not itself a principal.

Sixth, liveness over histories. Safety is
step-local; liveness is judged over traces,
by counters and timers against
designer-fixed thresholds, with progress
measured against aspiration, tied proxies,
and recorded epochs. Without it, a harness
that always halts is indistinguishable from
a harness that works.

Seventh, the meta-rule. Authority predates
the act it authorises and sits outside the
party it judges; revision is a recorded,
prospective, visible design act; where no
rule covers the case, inherited values freeze
rather than relax. Without it, every rule
above can be rewritten by the party it
constrains, and the theory certifies nothing.

Everything after the meta-rule is
application of the core, not addition to it.
Succession applies the meta-rule to the
designer role itself: authority passes only
by a rule fixed before the absence. Ending
applies it to the harness's own lifetime:
retirement is a design act, not a verdict.
Debts and chains apply it to obligations:
what the office owes survives the holder.
Identity applies it to revision: the traces,
not the code, are the harness. Trust,
anchors, and the root ledger apply it to the
record: a record's force comes from
provenance, chains, independent views, and
roots adopted before the dispute. None of
these adds a new kind of claim. Each takes a
core claim and applies it to the next thing
that could evade it. That is why the theory
could grow without changing its shape, and
why the governance chain can stop here
without leaving a gap: the next evasion
would be answered the same way, by the same
rule, one level up.

This gives the theory its scope test, and
the test is also a discipline. A proposed
addition earns its place only if it is either
a core claim above, or an application of one
to a new evasion, decided by the meta-rule. A
claim that is true of institutions in general
— of companies, courts, or clubs — but says
nothing about the designer-controlled
machinery between agent and world, does not
earn its place by being true. It earns it by
being forced: by showing that without it, some
harness failure cannot be attributed, some
evasion cannot be closed, or some principal
cannot be served. Generality is not a virtue
here. A theory of everything is a theory of
nothing, and the minimal core is the line
between a harness theory and a theory of
institutions wearing a harness's name.

What the core does not settle is separate,
and should not be confused with a gap in it.
What makes a harness good, rather than merely
working, and what code built from this model
looks like, are not evasions to be closed.
They are the next questions, and they are
answered by building and judging, not by
climbing another level.

The test for the whole section is again one a
later reader can apply. Take any claim in
this theory and ask: is it in the core, or
does it follow from the core by the
one-level-up move? If neither, it is
decoration, and the theory is better without
it. And take any claim of the core and ask:
if it were dropped, could the rest still
attribute a failure, close an evasion, or
serve a principal? If yes, the core is not
yet minimal, and the claim does not belong
in it. A theory that passes both tests is as
small as it can be while still saying what a
harness is.

## What each claim carries

Minimality is checked, not asserted. Drop one
claim, keep the other six, and ask what can no
longer be attributed, what evasion opens, or
what principal goes unserved. If nothing is
lost, the claim does not belong in the core.
Each drop below loses something the rest cannot
recover.

Drop the boundary. A failure loses its address.
A misleading system prompt, a changed retrieval
source, and a model error are all simply
outside the agent, so a badly designed harness
and an uncooperative world cannot be told
apart, and neither blame nor repair has a
target. Self-modification loses its attribution
with it: an agent rewriting its own prompt is
indistinguishable from the designer rewriting
it.

Drop the state split. The boundary still says
who controls what, but a transition over an
undivided state has no landing site: editing a
REPL variable and sending an email are both
just acting. The world-directed and harness-
directed distinction collapses, and authorship
loses its object, since there is no h to write.
A harness corrupting its own state and a world
changing look the same in the trace.

Drop authorship. The harness becomes a pipe:
observations in, actions out, with h fixed by
the designer alone. The agent choosing what to
see next, keeping notes, or installing a skill
has no description, and must be misread as a
policy change on the agent side or a changed
environment on the world side. A harness that
forbids self-editing and one that permits it
are the same harness, and agent-shaped future
observations become invisible to the theory.

Drop views. One state, one observation
function, presumably the agent's. The overseer
then either sees exactly what the agent sees,
which may be too little to judge or too much to
be safe, or is served by no function at all.
Sufficiency fails one way and separation the
other: showing a control can confer its use,
and a principal shown everything is served no
better than one shown nothing, because
judgement needs its own view, not the agent's.
Legibility collapses into transparency.

Drop arbitration. Principals keep their views,
but when two actions conflict, nothing decides
except order of arrival, the loudest principal,
or the agent itself, which makes the decider a
principal. Veto without compulsion goes with
it: the only resolutions left are compulsion,
forcing the preferred action, or collision,
letting both proceed. A conflict no rule covers
proceeds silently instead of suspending, so an
undecided conflict looks like a decision.

Drop liveness over histories. Judgement becomes
step-local: every step can be safe while the
run never completes, starves one principal, or
loops forever doing nothing forbidden. A
harness that halts safely and a harness that
works are indistinguishable. Vague goals become
unjudgeable with it: without aspiration and
epochs, progress demands either exact G,
unreachable when G shifts, or accepts any
halting trace as success.

Drop the meta-rule. The other six remain as
rules, but nothing stops the party they
constrain from rewriting them: the agent amends
arbitration in its own favour, resets the
thresholds it is judged against, or appoints
its own successor. An uncovered case is
improvised rather than frozen, so the gap
itself becomes the evasion. A theory whose
rules can be rewritten by the ruled certifies
nothing; it describes whatever happened.

Each drop loses a distinct thing: an address
for failure, a landing site for action, a
description of self-shaping, a principal's own
view, a decision that is not a principal, a
judgement over time, a rule about rules. No
loss is covered by the other six. The core is
minimal in the sense that matters: remove one
claim, and something a harness must do stops
being sayable.
## Membership: what counts as a harness, and what does not

The name travels. A test harness runs code
under test, an evaluation harness grades a
system after it has run, and a product is
called a harness because it contains one.
The core gives a test for membership that
does not depend on the name on the box, or
on whether the system lives in a terminal
or an editor. Packaging is not a claim.
Judge by function, not by product: one
system can contain a harness, an evaluator,
and a framework, and the test below applies
to each part, not to the whole at once.

A candidate is an agent harness exactly
when four things hold. Each is a core claim
applied to a running system, not a new
claim.

First, it runs a loop. It carries successive
observations and transitions for one policy
on one task, and the next step can depend
on the previous observation. A library offers
the parts of a loop; it runs none.

Second, the machinery is designer-controlled,
in the boundary sense, and outside the
policy's write set except for a permitted
authorship scope. A system the policy can
rewrite at will has no designer machinery
left to name.

Third, the policy can act through it. At
least one action lands somewhere: in the
world, or in harness state. Reading and
suggesting are not acting. Reshaping the
machinery the next observation will come
from is acting, even if no world state
changes yet.

Fourth, at least one designer-fixed control
can alter or suspend execution without the
policy's cooperation: a veto, a budget, a
verification, a containment, a stop rule, a
tool registry that admits only listed
actions. A log that records without altering
execution is not a control, and a cap the
policy can rewrite is not one either.

Weakness is not absence. A loop that appends
all history to the context, with no curation,
is a bad harness, not a non-harness.
Curation is a question of goodness, not of
membership. The same holds for a minimal
loop that runs the test suite and calls a
task done only if the suite passes: it is
an embryonic harness, not a demo, because
its check does not depend on the model's
word. No multi-agent architecture, no
learning, and no particular model is
required. A single policy in a loop is
enough, and swapping the model is a
designer act that leaves membership intact
so long as the boundary traces stay intact.

The neighbours fail one of the four, and
naming the failure is the test.

A framework composes agents. The loop,
tools, and control belong to each agent's
harness, not to the framework itself. A
framework embedding a harness in each agent
contains harnesses without being one. Where
a framework defines a joint policy and runs
its loop, it is the harness of that
composite policy, not merely a composer.

An SDK is material. It offers tool-calling
and loop primitives but imposes no running
loop. When it delivers the closed loop, with
context and control, it has crossed into
being a harness.

An autocomplete plugin suggests text at the
cursor. It keeps no task state, alters no
repository, and verifies nothing. An agent
inside an editor that takes a task, edits,
runs, and checks is a harness, in spite of
living where the plugin lives.

An orchestrator chains fixed steps. If the
observation never alters the course, there
is no loop, only a chain. A branch on an
observation — retry when a test fails —
turns that stretch of chain into a loop.

An evaluation or test harness runs an outer
loop over tasks or cases and grades after
the fact. It does not close the inner loop
for a single task, because it does not
decide the next step from the previous
observation; the system under test does.
It shares the machinery — control,
observation, records — but it is a
neighbouring sense of the same word, not
the same concept. A runner that launches an
agent harness contains one; the runner
itself is not one.

A bare system prompt, and a prompt with a
single model call, are harness-side
material, not a harness. They shape one
observation and run no loop. The claim that
a system prompt is harness, not agent, is a
claim about which side of the boundary it
sits on, not a claim that a prompt alone is
a harness.

Two corrections to a four-condition test in
the literature are forced by the core.
First, tools that alter only the external
environment are too narrow a test for
acting. In RLM, writing a REPL variable,
keeping a note, or installing a skill alters
no world state, yet it is the act that makes
the next observation different. The landing
site is h, not w, and a test that counts
only w misses the harness-directed half of
action altogether. RLM passes the test here
for the same reason: its REPL executor,
scoping, and recursion rules are
designer-fixed containment, and its
harness-directed actions shape what the root
model sees next.

Second, a control counts only by its author.
A cap, check, or guardrail that the policy
may rewrite at will, or that merely asks the
policy to cooperate, is a suggestion, not a
control. The meta-rule, not the presence of
a mechanism, is what makes a control a
control. A loop with an open shell, no
budget, no verification, and no stop rule
but the policy's own word trusts the model
in exactly the way the control condition
exists to exclude.

The test for the whole section is again one
a later reader can apply. For any candidate,
name the policy, the loop it runs, the
landing site of its actions, and the
designer-fixed control outside the policy's
write set. If any cannot be named, the
candidate is a neighbour, not a harness.
Passing the test is not passing judgement:
a harness that passes can still starve,
leak, cost too much, or serve its principal
badly. Membership is binary; goodness is
comparative, and it is the question the
core leaves open.

## A worked example: SWE-agent

SWE-agent takes a GitHub issue and a repository at the
buggy commit, and must produce a patch that fixes it
(Yang et al., 2024, arXiv:2405.15793). The account
below follows the system as it stands in its current
code and default configuration, not only the paper.
The paper's closed agent-computer interface has since
loosened into bash plus tool bundles, and that drift
is itself part of what the example teaches. Run the
membership test on it, and see what strains.

First, it runs a loop. `DefaultAgent.run`, in
`sweagent/agent/agents.py`, is literally
`while not step_output.done: step()`. Each step
queries the model on a history passed through the
configured history processors, parses a thought and
an action from the reply, executes the action in a
container environment, appends the observation, and
saves the trajectory. The next step can depend on
the previous observation: a test run in one step
changes what the model does in the next.

Second, the machinery is designer-controlled. The
system and instance templates, the tool bundles, the
history processors, the cost limits, the blocklist,
and the timeouts are configuration, in
`config/default.yaml` and the agent and model
configs, not model output. The policy cannot rewrite
them. Swapping the model is a designer act that
leaves membership intact, because the model is one
field of that configuration and the loop, the
templates, and the controls stay in place.

Third, the policy acts, on both sides of the split,
though unevenly. World-directed: editing repository
files, writing a reproduction script, running the
test suite. The repository is world as the object of
the task, even though the container that holds it
is a designer-owned sandbox; the boundary decides
which reading applies, as the state-split section
says. Harness-directed, in a thinner form than
RLM's: the agent writes no REPL variable and no
skill, but every step writes, through the harness,
its own trajectory, and that trajectory is what the
next observation is built from. Authorship here is
mostly world-mediated: changing a file changes the
test output and the file view the agent sees next.

Fourth, designer-fixed controls can alter or
suspend execution without the policy's
cooperation. The model config carries a
per-instance cost limit and a total cost limit;
exceeding either ends the run. A command times out
after 30 seconds, total execution after 1800
seconds, and repeated consecutive timeouts exit.
A blocklist refuses interactive and escape commands
— `vim`, `emacs`, `nano`, `nohup`, `gdb`, `less`,
`tail -f`, `make` as prefixes, and `python`,
`bash`, `sh` standing alone — and a blocked action
is not executed: the model is requieried, up to
three times, for a different action. Exceeding the
context window ends the run. None of these can be
rewritten by the policy.

The tower fits, and its top level is the
instructive part. Tokens parse, under the default
configuration's function-calling parser, into a
thought and an action; under older configurations
a thought-action parser did the same work, so a
move's identity is its parsed intent and landing
site, not its surface form. The moves are the
built-in `bash` command plus the configured
bundles — in the default configuration, a
registry, an editor, and a review-on-submit bundle.
Because `bash` is a move, `A(h)` is not a closed
list: any shell string is a candidate move, minus
the blocklist, minus the remaining budget and time.
Control here is therefore veto plus cost, not
admission. The membership section's example of a
registry admitting only listed actions is one kind
of control, not the only kind. SWE-agent passes by
the other kinds, and the difference matters for
goodness: a veto blocks known bad commands, it
does not bound in advance what a shell string can
do.

Observation is a function over the whole
trajectory, not only over the container's current
state. The next-step template is simply
`OBSERVATION: {{observation}}`, truncated at
100,000 characters with the elided count noted.
The default history processor is the identity, but
the classic `LastNObservations` processor, used in
the 0.7 configuration with five observations kept,
replaces older observations with the line
`Old environment output: (n lines omitted)`. A
designer-fixed processor thus rewrites history
between steps, changing future observations
without any agent action. A POMDP absorbs this
only because `h` in the code includes the history
and trajectory lists themselves, not just the
container state. If `h` were only the container,
history rewriting would be unstateable.

Ending and verdict come apart here, and the split
is worth keeping. The loop ends — `done` becomes
true — on submit, on forfeit, on cost, context, or
timeout, or by autosubmission after an error, where
the harness extracts the current diff from the
last trajectory step's state and submits that.
A run can therefore end as `submitted` without
ever having succeeded. Success is judged outside
the loop: the SWE-bench evaluation hook runs the
maintainer's test suite afterwards and grades the
patch. That is the evaluation harness of the
membership section, a neighbour, not part of the
inner loop. Inside the loop, by contrast, the
review-on-submit bundle is a designer-fixed check:
on submit, the harness shows the model its own
diff and asks it to rerun the reproduction script,
remove that script, revert any test files it
touched, and submit again. Stopping, being halted,
and being judged are three different events, and
SWE-agent keeps them in three different places.

What it lacks is as informative as what it has.
There is no live view for a second principal: an
overseer gets the trajectory file and the
trajectory inspector after the run, not an
observation function of its own during it. A
human model exists, but it replaces the policy
rather than intervening in a run already under
way, so human intervention stays an open question.
And there is no arbitration, because there is a
single policy: the only conflicts are vetoes, a
blocklist refusing a command, not principals
contending over a transition.

The test a later reader can apply is satisfied:
name the policy, the model in the configuration;
the loop, `run` calling `step` until `done`; the
landing sites, repository files and the trajectory;
the controls, cost, timeout, and blocklist outside
the policy's write set. SWE-agent passes. Passing
is not goodness: the open shell that makes it
flexible is exactly what makes its control
reactive rather than bounding, and a harness
that passes can still cost too much, starve, or
submit a patch that fails every test.

The stub adapter in `harness-core/src/adoption.rs`
names these same items in the core's terms: the
loop, the action space, the state split, the
views, the controls, and the ending, with the
membership result and the gaps above stated
plainly. A second adoption, from a different
category, can be compared against this one in
those same terms, and where the two disagree,
the disagreement itself is the finding.

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
  harness built on the RLM abstraction with a persistent IPython
  REPL and a continual harness. §2.5's Continual Harness, with
  its immutable base prompt and versioned, rollbackable
  supplemental state, is the source of the fixed-core /
  mutable-supplement split in the boundary section.
- Classic reinforcement learning and decision theory: Sutton and
  Barto, and the older MDP / POMDP literature. The point is not
  novelty, it is whether the old formalism already contains the
  distinctions we need, once it is applied carefully. Ch. 3's
  boundary rule (control, not knowledge, draws the line) is the
  source of the designer-control test above.
- Supervisory control: Ramadge and Wonham, "Supervisory
  Control of a Class of Discrete Event Processes", SIAM
  Journal on Control and Optimization, 1987. A supervisor
  disables a subset of controllable events and cannot force
  an event to occur. That veto-without-compulsion shape is
  the source of the arbitration section above: a harness
  arbitrator disables contested transitions, it does not
  compel the action it would prefer.
- Safety and liveness: Bowen Alpern and Fred B. Schneider,
  "Defining Liveness", Information Processing Letters
  21(4), October 1985. Every trace property is the
  conjunction of a safety property and a liveness
  property. That decomposition is the source of the
  liveness section above: precedence arbitration is
  safety, step-local, so liveness must be judged over
  histories, and Ramadge and Wonham's nonblocking
  supervisor is the finite-state precedent for that
  judgement.
- Vague goals and aspiration: Herbert A. Simon, "A
  Behavioral Model of Rational Choice", Quarterly
  Journal of Economics, 1955. Satisficing — search
  stops at the first option meeting an aspiration
  level, rather than at an optimum — is the source
  of the progress section's aspiration in place of
  a point, and of its treatment of adjusting
  aspirations as shifting goals.
- Succession fixed before vacancy, as illustration, not
  authority: the United States Presidential Succession Act of
  1947 and the Twenty-fifth Amendment, which fix an order in
  advance; and the Linux kernel's succession and continuity
  plan, which fixes a timed process with an initiator and a
  backup initiator. In both, legitimacy comes from the rule
  fixed before the vacancy, not from the successor's claim.
- Software sunset practice, as precedent for announced, drained
  endings, not as authority: deprecation announced before shutdown,
  with a migration path and a date (IETF, RFC 8594, the Sunset
  header). The staged shape — announce, drain, then stop — is the
  source of the ending section's drain and announcement rules.
- Novation in contract law, as illustration, not authority: a new
  party steps into a contract only by agreement, and takes its
  obligations, not just its records. That assets-and-liabilities
  shape is the source of the successor section's fourth
  inheritance: outstanding obligations pass with the office.
- Trending and historical harness implementations on GitHub, to be
  read as examples, not as authorities. An implementation that
  works is evidence, not a theory.
- Corporate successor liability, as contrast, not authority: the
  traditional rule is that a buyer of assets does not inherit
  the seller's debts, except for express assumption, de facto
  merger, mere continuation, or fraud to escape liability. The
  chain section inverts that default: a harness successor claims
  the office, not the assets, so continuation is the rule, not
  the exception.
- Identity across revision, as illustration, not authority:
  Plutarch, Life of Theseus, on the ship whose planks were
  replaced one by one, and Hobbes's second ship built from
  the discarded planks; and H. L. A. Hart, The Concept of
  Law, 1961, on the rule of recognition, the secondary rule
  by which a system's own rules are identified as its rules.
  The identity section's recognition rules, and its
  functional test for what counts as constitutive, are
  drawn from that shape, not from Hart's authority.
- Tamper-evident records, as illustration, not authority:
  hash-chained append-only logs and Merkle trees, as in
  version control and Certificate Transparency (IETF,
  RFC 6962). A chain whose later links are held by many
  parties, and anchored outside the harness, is the source
  of the trust section's graded trust and its bound on
  forgery. Consistency proofs between views of one log,
  gossiped among independent monitors, are the source of
  the ledger section's claim that a log's integrity is a
  property of distribution: a fork is visible as a
  disagreement between views, without any root above
  the log.
- Trust roots, as illustration, not authority: a root
  certificate in public-key infrastructure is installed by
  the relying party, not derived from a chain. The chain
  ends where the relying party's own act begins, which is
  the source of the regress section's adopted root.
- Trust anchor rotation, as illustration, not authority:
IETF, RFC 5011, "Automated Updates of DNS Security (DNSSEC)
Trust Anchors", September 2007. A stand-by key, published and
signed under the old key, is watched for a hold-down period
before it is trusted; the old key retires by signed revocation.
Recovery needs at least one uncompromised anchor; with none,
only a manual, out-of-band update remains. That shape is the
source of the rotation section's overlap, closing link, and
emergency rule.
- Agent harness definition, as foil and test, not authority:
Sanderson Oliveira de Macedo, "What makes a harness a
harness: necessary and sufficient conditions for an agent
harness", arXiv:2606.10106, August 2026. Its four
conditions — a reasoning, action, and observation loop, a
tool interface, context management, and a control
mechanism independent of the model's obedience — and its
boundary against framework, SDK, IDE plugin, eval harness,
and orchestrator, are the source of the membership
section's neighbours and of its inner-loop test. The
membership section departs from it in two places: actions
landing in harness state count as acting, not only actions
altering the external environment, and a control counts
only if it is designer-fixed and outside the policy's
write set, so that weak context management is badness,
not absence.
- SWE-agent, as worked example, not authority:
John Yang et al., "SWE-agent: Agent-Computer
Interfaces Enable Automated Software Engineering",
arXiv:2405.15793, 2024, and the system's current
code and default configuration, inspected in
`sweagent/agent/agents.py`, `sweagent/tools/tools.py`,
`sweagent/agent/history_processors.py`, and
`config/default.yaml`. The loop, the bash-plus-
bundles action set, the cost, timeout, and blocklist
controls, and the separation of submit from
SWE-bench grading are the source of the worked
example section.
- Planning under partial observability, as the
formalism judged, not authority: Leslie Pack
Kaelbling, Michael L. Littman, and Anthony R.
Cassandra, "Planning and acting in partially
observable stochastic domains", Artificial
Intelligence, 1998. A POMDP policy maps
histories, or beliefs over states, to actions;
the formalism itself says nothing about who
fixes its transition and observation
functions, which is the source of the POMDP
section's division of labour.
- Temporally extended actions, as complement,
not authority: Richard S. Sutton, Doina Precup,
and Satinder Singh, "Between MDPs and
semi-MDPs: a framework for temporal
abstraction in reinforcement learning",
Artificial Intelligence, 1999. An option runs
until a termination condition, then returns
control, which is the source of the POMDP
section's treatment of a subagent move as a
semi-Markov action at the outer level.
