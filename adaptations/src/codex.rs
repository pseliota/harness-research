//! Stub adapter for OpenAI Codex.
//!
//! Codex is OpenAI's coding agent, available as a CLI,
//! IDE extension, and cloud agent. This adapter names,
//! in the core's terms, what adapting it would involve.
//! It does not run Codex, and its descriptions are stubs
//! to be verified against Codex's actual code and
//! documentation.

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// Codex, adopted on top of the core.
pub struct Codex;

impl Codex {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Sandbox boundaries: what the default sandbox admits and refuses, and whether those bounds are designer-fixed controls or user configuration.",
            "Cloud versus local runs: whether the loop, views, and controls are the same in the CLI, IDE, and cloud forms, or three different harnesses sharing a name.",
            "Approval model: which actions require approval in each mode, and how approvals are recorded.",
        ]
    }
}

impl AdoptedHarness for Codex {
    fn name(&self) -> &str {
        "Codex"
    }

    fn policy(&self) -> &str {
        "An OpenAI model queried on a context history shaped by the system prompt, project instructions, and prior tool results."
    }

    fn loop_description(&self) -> &str {
        "Loop: observe the context, choose text or a tool call, execute it, typically inside a sandbox, append the observation, and repeat until the task is done or the run is stopped."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "read and search files".to_string(),
            "edit and write files, within sandbox bounds".to_string(),
            "run shell commands, within sandbox bounds".to_string(),
            "delegate to cloud agents, where that mode is used".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, sandbox configuration, tool registry, and approval settings.".to_string(),
        )
    }

    fn views(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: terminal or IDE output, diffs, and summaries.".to_string(),
            "Overseer: run transcript, cost, and sandbox and approval records.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Sandbox containment: execution is bounded by the sandbox, not by the model's word alone.".to_string(),
            "Approvals for actions outside the allowed set.".to_string(),
            "User stop and interruption of a running loop.".to_string(),
        ]
    }

    fn ending(&self) -> String {
        "The loop ends when the task is complete, when the user stops it, or when a sandbox or approval control refuses a required action.".to_string()
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
