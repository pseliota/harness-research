//! Stub adapter for Gemini CLI.
//!
//! Gemini CLI is Google's terminal coding agent. This
//! adapter names, in the core's terms, what adapting
//! it would involve. It does not run Gemini CLI, and
//! its descriptions are stubs to be verified against
//! its actual code and documentation.

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// Gemini CLI, adopted on top of the core.
pub struct GeminiCli;

impl GeminiCli {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Arbitration: whether any designer-fixed rule decides between contending principals' proposals, or whether conflicts reduce to approvals and user stop, has not been verified against its code.",
            "Sandbox and approval model: what bounds execution, and which actions require approval.",
            "Tool and extension model: how extensions change the action space, in the core's A(h) terms.",
            "Multi-model and multi-tool behaviour: whether different models or tools change the loop, or only its parameters.",
        ]
    }
}

impl AdoptedHarness for GeminiCli {
    fn name(&self) -> &str {
        "Gemini CLI"
    }

    fn policy(&self) -> &str {
        "A Gemini model queried on a context history shaped by the system prompt, project instructions, and prior tool results."
    }

    fn loop_description(&self) -> &str {
        "Loop: observe the context, choose text or a tool call, execute it against the repository, append the observation, and repeat until the task is done or the run is stopped."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "read and search files".to_string(),
            "edit and write files".to_string(),
            "run shell commands".to_string(),
            "use extensions, which change the available moves".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, extension registry, tool registry, and approval configuration.".to_string(),
        )
    }

    fn views(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: terminal output, diffs, and summaries.".to_string(),
            "Overseer: run transcript and cost records, where recorded.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Arbitration: not yet stated — whether Gemini CLI has a designer-fixed rule deciding between contending principals' proposals has not been verified against its code, and no such rule is claimed here.".to_string(),
            "Approvals for actions outside the allowed set.".to_string(),
            "Tool registry: only listed tools and extensions are available moves.".to_string(),
            "User stop and interruption of a running loop.".to_string(),
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
