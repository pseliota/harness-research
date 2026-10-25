# Theory

This document states the core of the theory. It states only what is needed to define a harness. It states only what is needed to test a candidate. It states only what is needed to adopt an existing harness. Everything else is out, even if it is interesting elsewhere.

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
- A principal is a party with its own goal, observation function, and write permission set, such as the user, the agent, or the overseer.
- p_i(o_i | s) stands for the observation function for principal i, where o_i is the observation of principal i.

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

A retrieval result is an observation, not an action. A chat reply lands in the transcript in h, for another principal to see. The state component and the audience differ in that case. Unparseable token sequences are no action at all, not failed actions. The harness still responds, with an error, a retry, or a halt. That response follows a transition rule, not a rule in the action space.

Invert the cut: the harness code is the policy, and the model is the environment. The same levels of abstraction apply, and the two roles exchange places.

A(h) takes h only, not w. An attempt is a decision of the harness. The harness decides from h. A world offer reaches the harness as an observation. That observation then enters h as recorded history. A cached page in h still offers a move the world no longer offers. The harness still attempts the move, and the move fails. The attempt follows h. Success follows w.

## POMDP as the shape of one step

For one step, a POMDP fits. For a harness, it does not fit. A POMDP is the shape of a step, not the theory of the steps. The agent never sees w directly, and much of what it sees is designed h. But a POMDP is one tuple: one state set, one action set, one observation set, one transition function, one observation function, and one reward.

1. First, a harness has not one action set. The levels of abstraction give a family of POMDPs, one per level, and the parser and expansion link the levels. A rejected token sequence is in neither action set. A subagent move spans many steps, so it is not a one-step move.
2. Second, a harness has not one observation function. Each principal has its own p(o | s) over the same s. The user, the agent, and the overseer see different observations of the same state.
3. Third, the functions of a POMDP stay fixed, but the functions of a harness do not. Within a period in which the harness code stays fixed, a run is a POMDP. Across such periods, a run is a family of POMDPs, one for each recorded design decision. Registry, parser, and thresholds change by later design decisions. A blocklist is a veto, not a price: it forbids a move, it does not add a cost.
4. Fourth, the observations of a POMDP are evidence, but the observations of a harness are designed artefacts. SWE-agent history processor replaces older observations with an omission note. The next observation comes from the rewritten trajectory. This fit holds only because trajectory lists sit in h. A belief from a designed record is a belief about what the designer chose to show, not about w.

Finally, the vacuity point: if s and o each hold the whole history, any process is Markov. Then the claim "a run is a POMDP" is true and empty. The claim gains content only for a fixed s. For a fixed s, state which POMDP, which level, which principal, and which vetoes apply. Keep the POMDP as local shape, for one level. The rest of the theory goes around it.

## The loop

One loop serves one policy on one task. Each step observes, chooses, transitions, and observes again. The next step depends on the previous observation. The loop repeats until its stop rule fires. A run terminates when the stop rule fires. Termination and success stay separate. A run can terminate without success.

## Observation functions and precedence

Each principal i receives data through p_i(o_i | s). The function gives enough data for the job of principal i. The function gives no write permission through shown data.

When principals contend over one transition, a designer-fixed precedence rule decides the conflict. Veto power blocks an action and never forces one. The rule shows its decision to the principal it preempts. An action that no precedence rule decides stays suspended. A harness with no contending principals has no precedence rule. A veto in such a harness still counts as a control.

Designer-fixed controls also include approval, a budget cap, and interrupt. A control alters or suspends execution without policy cooperation. A rewritable cap counts as a suggestion, not a control. A cooperative guardrail counts as a suggestion, not a control.

## The minimal core

Seven claims form the core. A proposed claim joins the core only when it passes one test. Drop the claim and keep the core. Ask whether a failure then lacks an address, an adoption then lacks a name, or a principal then lacks service. Admit the claim only when the answer to one question is yes. A claim true of institutions in general fails the test when it says nothing about designer-controlled machinery. Questions about what makes a harness good fail the test.

1. Loop. One loop serves one policy on one task, and the next step depends on the previous observation.
2. Boundary. The boundary separates agent, harness, and world by who controls what. The designer controls the harness. The agent controls its own moves. Neither the designer nor the agent controls the world.
3. State split. The state split separates every state into w and h. A move changes w, h, or both, and the split names which part changes.
4. Action levels. Each level of abstraction has its own action set. The set of moves available to attempt is A(h), a function of h only.
5. Write permission. The agent holds write permission for part of h inside a scope the designer permits. That permission shapes later observations and transitions. The fixed core and the modification policy stay outside the agent write set.
6. Observation functions. Each principal i receives data through p_i. The function gives enough data for the job of principal i. The function gives no write permission through shown data.
7. Precedence and controls. When principals contend over one transition, a designer-fixed precedence rule decides the conflict, by veto, never by compulsion, and shows its decision to the preempted principal. At least one designer-fixed control alters or suspends execution without policy cooperation.

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

## Adoption: mapping an existing harness onto the core

An existing harness is adopted by mapping it onto the core. The adapter names its policy, its loop, its action space A(h), its state split, its observation functions, its controls, its termination, and its membership result. A gap is a place where the fit is uncertain, stated plainly, not forced.

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
2. What distinguishes a good harness from a merely working one, once cost, robustness, legibility to humans, and failure modes are included, not just reaching the task goal?
3. Is agent control over observation the central principle, or one principle among several, alongside control over transitions and legibility?
4. How should a theory handle multiple principals at once: the agent, the user, and the human overseer, each needing their own observation function?
5. What invariants survive across good harnesses, across models, and across tasks?
6. How can a formalism be useful now and into the future, without knowing the future? What should it commit to, and what should it leave open, so it can be revised gracefully?
7. What would a harness look like if it were coded from a solid mental model, so cleanly that new developments fit into it without tangling, and maintenance stays low?

## Sources to read, test, and argue with

- Prime Agent, Prime Intellect, August 2026, a self-improving harness on the Recursive Language Model abstraction. It gives the fixed-core and mutable-supplement split.
- Classic reinforcement learning and decision theory: Sutton and Barto, and the older MDP / POMDP literature. It gives the designer-control test.
- Supervisory control: Ramadge and Wonham, "Supervisory Control of a Class of Discrete Event Processes", SIAM Journal on Control and Optimization, 1987. It gives the veto-without-compulsion shape for the precedence rule.
- Agent harness definition, as foil and test, not authority: Sanderson Oliveira de Macedo, "What makes a harness a harness: necessary and sufficient conditions for an agent harness", arXiv:2606.10106, August 2026. Its four conditions and neighbour boundary serve as foil and test for the membership section. This document departs from it in three places. A harness-state action counts as an action. A control counts only if designer-fixed and outside policy write permission. A loop that appends all history with no curation counts as a bad harness, not as a non-harness, although Macedo excludes it for lack of active context selection.
- SWE-agent, as worked example, not authority: John Yang et al., "SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering", arXiv:2405.15793, 2024, and the system's current code and default configuration. Its loop, action set, and controls serve as the worked example.
- Planning under partial observability, as the formalism judged, not authority: Leslie Pack Kaelbling, Michael L. Littman, and Anthony R. Cassandra, "Planning and acting in partially observable stochastic domains", Artificial Intelligence, 1998. It gives the division of labour for the POMDP section.
- Temporally extended actions, as complement, not authority: Richard S. Sutton, Doina Precup, and Satinder Singh, "Between MDPs and semi-MDPs: a framework for temporal abstraction in reinforcement learning", Artificial Intelligence, 1999. It gives the treatment of a subagent move as a semi-Markov action at the outer level.
- Trending and historical harness implementations on GitHub, to be read as examples, not as authorities. A working implementation counts as evidence and does not by itself give a theory.
