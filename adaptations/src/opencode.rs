//! Stub adapter for OpenCode.
//!
//! OpenCode is an open-source, provider-agnostic
//! terminal coding agent. This adapter names, in the
//! core's terms, what adapting it would involve. It
//! does not run OpenCode, and its descriptions are
//! stubs to be verified against its actual code and
//! documentation.
//!
//! One part is verified, not merely stubbed: human
//! intervention, checked against anomalyco/opencode,
//! dev branch, commit 17b79f4e0c16dcff200b0417567ed22aeb2d7166
//! (2026-10-24). See `controls` and `gaps` below, and
//! `controls` below. Three
//! of its former gaps are now verified, not open:
//! interruption settlement (run-coordinator),
//! partial-step resumption (step abort, new step on
//! the next drain), and intervention precedence
//! (per-mechanism, not a ranking of principals:
//! interrupt over in-flight work, execution over
//! edit, steer over queue, deny over move, reply
//! over pending move; user versus shutdown is
//! decided by arrival order, not rank).

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// OpenCode, adopted on top of the core.
pub struct OpenCode;

impl OpenCode {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Provider-agnostic policy: whether swapping the model changes only the policy, or also the loop, views, or controls, in the core's terms.",
            "Build and plan agents: whether these are two loops, two action spaces, or one loop with two modes.",
            "LSP and SDK surface: which parts belong to the harness, and which belong to tooling around it.",
        ]
    }
}

impl AdoptedHarness for OpenCode {
    fn name(&self) -> &str {
        "OpenCode"
    }

    fn policy(&self) -> &str {
        "A model chosen by the user, queried on a context history shaped by the system prompt, project instructions, and prior tool results."
    }

    fn loop_description(&self) -> &str {
        "Loop: observe the context, choose text or a tool call, execute it against the repository, append the observation, and repeat until the task is done or the run is stopped."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "read and search files, including via language-server information".to_string(),
            "edit and write files".to_string(),
            "run shell commands".to_string(),
            "delegate to build or plan subagents".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, provider and model configuration, tool registry, and session state.".to_string(),
        )
    }

    fn observation_functions(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: terminal output, diffs, and summaries.".to_string(),
            "Overseer: session transcript and cost records, where recorded.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Precedence: no ranking of principals — intervention precedence is per mechanism: interrupt over in-flight work, active execution over revert (busy error), steer over queue at promotion, permission deny over a move, and a user reply over the whole pending permission batch; user versus shutdown is decided by arrival order, since the coordinator's only interrupt reasons are user and shutdown.".to_string(),
            "Approvals for actions outside the allowed set.".to_string(),
            "Tool registry: only listed tools are available moves.".to_string(),
            "User interrupt of a running session (Session.interrupt, reason 'user'; acknowledged on acceptance, cleanup settles asynchronously; idle interrupt is a no-op; a second interrupt while stopping, or in the settlement window, returns false and claims recorded wakes; wakes during cleanup start a successor at settle; awaitIdle waits for settlement).".to_string(),
            "Partial-step resumption restarts the step: an interrupted step is failed as aborted, unsettled tools fail, completed tool results survive in history, and the next drain starts a new step; the 'continue from where you left off' observation belongs to interrupted provider streams recovered inside a step, not to user interruption.".to_string(),
            "Steered input delivered at a step boundary, queued input held for the next turn, and session revert staged only while idle.".to_string(),
            "Agent or model switch published as a selection event, with the session and its history surviving.".to_string(),
        ]
    }

    fn termination(&self) -> String {
        "The loop terminates when the task is complete, when the user stops it, or when a control refuses a required action.".to_string()
    }

    fn membership(&self) -> Candidate {
        Candidate {
            runs_loop: true,
            designer_controlled: true,
            policy_can_act: true,
            has_designer_control: true,
        }
    }
}
