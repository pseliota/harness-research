//! Stub adapter for OpenAI's Codex CLI only.
//!
//! The CLI is the open-source part, under Apache 2.0.
//! Codex's IDE extension and cloud service are not
//! open source, so they are out of scope for this
//! adapter.
//!
//! Sandbox and approval, below, are verified in
//! outline, not merely stubbed: checked against
//! openai/codex, commit
//! 970111f6cb62cc02fe36a82ea7d71b37a2889c97,
//! in `codex-rs/protocol/src/config_types.rs`
//! (`SandboxMode`), `codex-rs/protocol/src/protocol.rs`
//! (`SandboxPolicy`, `AskForApproval`),
//! `codex-rs/protocol/src/models.rs`
//! (`SandboxPermissions`), and
//! `codex-rs/execpolicy/src/decision.rs`
//! (`Decision`). The loop description and views
//! are still summaries, not checked line by line,
//! and are listed as gaps. This adapter does not
//! run Codex.

use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

/// Codex CLI, adopted on top of the core.
pub struct CodexCli;

impl CodexCli {
    /// Where the fit is uncertain, or still needs verifying.
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "Per-OS sandbox enforcement: the policy shape below was checked, but how each platform enforces it (macOS seatbelt, Linux landlock, Windows sandbox) was not verified here.",
            "Approval and exec-policy amendment records: whether approvals persist across sessions, and where they are recorded, was not verified.",
            "Loop, views, and overseer records: the loop description and views below are summaries, not checked line by line against the code.",
            "Per-command escalation and arbitration: a move may request wider permissions, gated by user approval. Whether that is arbitration in the theory's sense, or only a veto, is not settled here.",
            "Scope: the IDE extension and cloud service are excluded here, as they are not open source; do not read this adapter as covering them.",
        ]
    }
}

impl AdoptedHarness for CodexCli {
    fn name(&self) -> &str {
        "Codex CLI"
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
            "edit and write files, within the sandbox policy's writable roots".to_string(),
            "run shell commands, whose permitted scope varies with the sandbox policy and the exec-policy rules".to_string(),
            "per-command permission requests: a shell move may ask to run outside the sandbox, or to stay sandboxed with widened permissions for that command only, subject to approval".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: repository files, test outcomes, and external services, as the object of the task.".to_string(),
            "Harness: conversation history, sandbox policy and writable roots, exec-policy rules, approval mode, and tool registry.".to_string(),
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
            "Sandbox policy: read-only (the default), workspace-write (the working directory plus configured writable roots, with read-only subpaths protecting paths whose modification could escalate privilege, for example .git/hooks), external-sandbox (full disk access inside an existing external sandbox), or danger-full-access (no restrictions). Network access is off by default in read-only and workspace-write.".to_string(),
            "Exec-policy rules: allow, prompt, or forbidden decisions per command. A forbidden command is blocked without further consideration.".to_string(),
            "Approval modes: untrusted (approval required unless an exec-policy rule allows the command), on-request (the default; the model decides when to ask), granular (per-category allow, or automatic rejection instead of a prompt, for sandbox approvals, policy-rule prompts, skill approvals, and permission requests), or never (never ask; a command needing approval is rejected outright and the failure returns to the model).".to_string(),
            "Approval-seeking is not itself the control under the default on-request mode: the model decides when to ask, so the controls that hold regardless are sandbox enforcement and forbidden exec-policy rules, not the asking.".to_string(),
            "User interrupt of a running task: a session interrupt aborts the current task, if any.".to_string(),
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
