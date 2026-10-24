//! Stub adapter for Claude Code.
//!
//! Claude Code is a terminal-first coding agent, with IDE
//! extensions. This adapter names, in the core's terms,
//! what adapting it would involve. It does not run Claude
//! Code, and its descriptions are stubs to be verified
//! against Claude Code's actual code and documentation.

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// Claude Code, adopted on top of the core.
pub struct ClaudeCode;

impl ClaudeCode {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Exact permission model: which actions prompt, which are allowed by configuration, and how approvals are recorded — to verify against the current permission system.",
            "Subagent and skill scoping: how a subagent's action space and views differ from the root agent's, in the core's tower terms.",
            "Live overseer view: what, if anything, a second principal sees during a run, as opposed to after it, in the transcript.",
        ]
    }
}

impl AdoptedHarness for ClaudeCode {
    fn name(&self) -> &str {
        "Claude Code"
    }

    fn policy(&self) -> &str {
        "A Claude model queried on a context history shaped by the system prompt, project instructions, skills, and prior tool results."
    }

    fn loop_description(&self) -> &str {
        "Loop: observe the context, choose text or a tool call, execute it against the repository, append the observation, and repeat until the task is done or the user stops it."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "read and search files".to_string(),
            "edit and write files".to_string(),
            "run shell commands".to_string(),
            "spawn subagents".to_string(),
            "use skills, which parametrise the action space rather than acting themselves".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, plan and task state, skills and memory, tool registry, and permission configuration.".to_string(),
        )
    }

    fn views(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: terminal output, diffs, and summaries in the terminal or editor.".to_string(),
            "Overseer: transcript, cost, and permission prompts, as recorded during the run.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Permission prompts before risky actions, such as edits and shell commands outside the allowed set.".to_string(),
            "Tool registry: only listed tools are available moves.".to_string(),
            "User stop and interruption of a running loop.".to_string(),
        ]
    }

    fn ending(&self) -> String {
        "The loop ends when the task is complete, when the user stops it, or when a control refuses a required action and no alternative is chosen.".to_string()
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
