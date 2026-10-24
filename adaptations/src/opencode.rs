//! Stub adapter for OpenCode.
//!
//! OpenCode is an open-source, provider-agnostic
//! terminal coding agent. This adapter names, in the
//! core's terms, what adapting it would involve. It
//! does not run OpenCode, and its descriptions are
//! stubs to be verified against its actual code and
//! documentation.

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

    fn views(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: terminal output, diffs, and summaries.".to_string(),
            "Overseer: session transcript and cost records, where recorded.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Approvals for actions outside the allowed set.".to_string(),
            "Tool registry: only listed tools are available moves.".to_string(),
            "User stop and interruption of a running loop.".to_string(),
        ]
    }

    fn ending(&self) -> String {
        "The loop ends when the task is complete, when the user stops it, or when a control refuses a required action.".to_string()
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
