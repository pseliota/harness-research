//! Stub adapter for Cursor, agent part only.
//!
//! Cursor is an editor with an agent inside it. The
//! membership test applies to the agent part, not to
//! the editor as a whole: an editor that merely suggests
//! text is a neighbour, an agent that takes a task,
//! edits, runs, and checks is a harness, even though it
//! lives where the plugin lives. This adapter names,
//! in the core's terms, what adapting the agent part
//! would involve. It does not run Cursor, and its
//! descriptions are stubs to be verified.

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// Cursor's agent, adopted on top of the core.
pub struct Cursor;

impl Cursor {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Editor versus agent boundary: which parts of Cursor's behaviour belong to the agent harness, and which belong to the editor or to a plugin.",
            "Plan, ask, and agent modes: whether these are different loops, different action spaces, or the same loop with different controls.",
            "Cloud agents: whether a cloud run is the same harness as a local editor run, or a different one sharing a name.",
        ]
    }
}

impl AdoptedHarness for Cursor {
    fn name(&self) -> &str {
        "Cursor (agent)"
    }

    fn policy(&self) -> &str {
        "A model queried on a context history shaped by the editor state, project instructions, and prior tool results."
    }

    fn loop_description(&self) -> &str {
        "Loop: observe the context, choose text or a tool call, execute it against the repository, append the observation, and repeat until the task is done or the user stops it."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "read and search files in the editor".to_string(),
            "edit and write files, shown as editor diffs".to_string(),
            "run terminal commands".to_string(),
            "ask the user a clarifying question, as a move whose landing site is the transcript".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, editor state and diffs, tool registry, and agent mode configuration.".to_string(),
        )
    }

    fn views(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: editor chat, diffs, and terminal output.".to_string(),
            "Overseer: run transcript and cost records, where recorded.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "User review of diffs before they are accepted.".to_string(),
            "Tool registry: only listed tools are available moves.".to_string(),
            "User stop and interruption of a running loop.".to_string(),
        ]
    }

    fn ending(&self) -> String {
        "The loop ends when the task is complete, when the user accepts or rejects the changes, or when the user stops it.".to_string()
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
