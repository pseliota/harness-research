//! Stub adapter for GitHub Copilot, agent part only.
//!
//! Copilot includes autocomplete, chat, and an agent
//! that takes tasks, edits, and runs checks, including
//! as a coding agent on pull requests. The membership
//! test applies to the agent part, not to autocomplete
//! alone, and not to the editor or hosting platform as
//! a whole. This adapter names, in the core's terms,
//! what adapting the agent part would involve. It does
//! not run Copilot, and its descriptions are stubs to
//! be verified.

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// Copilot's agent, adopted on top of the core.
pub struct Copilot;

impl Copilot {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Autocomplete versus agent boundary: which parts of Copilot's behaviour belong to the agent harness, and which are suggestions only.",
            "Pull-request agent versus IDE agent: whether these are the same harness in different views, or different harnesses sharing a name.",
            "Review and merge controls: who, in the core's terms, arbitrates when the agent's proposed changes conflict with a human reviewer's.",
        ]
    }
}

impl AdoptedHarness for Copilot {
    fn name(&self) -> &str {
        "GitHub Copilot (agent)"
    }

    fn policy(&self) -> &str {
        "A model queried on a context history shaped by the repository, the task or pull request, and prior tool results."
    }

    fn loop_description(&self) -> &str {
        "Loop: observe the context, choose text or a tool call, execute it against the repository, append the observation, and repeat until the task is done, the changes are proposed, or the run is stopped."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "read and search files".to_string(),
            "edit and write files, proposed as diffs".to_string(),
            "run terminal commands and checks".to_string(),
            "propose a pull request, as a move whose audience is a human reviewer's view".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, proposed diffs, tool registry, and review configuration.".to_string(),
        )
    }

    fn views(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: editor or pull-request diffs and summaries.".to_string(),
            "Overseer: run transcript, review comments, and merge records.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Human review of proposed diffs before merge.".to_string(),
            "Tool registry: only listed tools are available moves.".to_string(),
            "User stop and interruption of a running loop.".to_string(),
        ]
    }

    fn ending(&self) -> String {
        "The loop ends when the task is complete and changes are proposed, when a reviewer rejects them, or when the run is stopped.".to_string()
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
