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
//! (`Decision`). Escalation, below, is likewise
//! verified in outline, not merely stubbed:
//! checked against the same commit, in
//! `codex-rs/core/src/tools/handlers/request_permissions.rs`,
//! `codex-rs/core/src/session/mod.rs`
//! (`request_permissions_for_environment`,
//! `persist_execpolicy_amendment`),
//! `codex-rs/core/src/tools/approvals.rs`
//! (`request_guardian_approval`), and
//! `codex-rs/protocol/src/protocol.rs`
//! (`ReviewDecision`). The loop description and
//! views are still summaries, not checked line by
//! line, and are listed as gaps. This adapter
//! does not run Codex.

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
            "Guardian reviewer's deciding scope: an escalation request is decided by the automated guardian reviewer when it decides, and otherwise goes to the user as an event; what the guardian is configured to decide on its own, rather than pass on, was not verified here.",
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
            "per-command permission requests: the agent may call a request_permissions tool asking for wider permissions for the turn, or for the session, decided as described under controls".to_string(),
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
            "Escalation, agent-initiated: the agent calls a request_permissions tool asking for wider permissions; the request is decided by the automated guardian reviewer when it decides, and otherwise goes to the user as an event to answer. A grant is scoped to the turn, or to the session when the user approves for the session; it is not written to disk. Under approval mode never, or a granular configuration that disallows permission requests, the request returns an empty grant: escalation is refused without asking.".to_string(),
            "Escalation is not arbitration in the theory's sense: no designer-fixed rule over action pairs decides it. It is a judging party, guardian or user, allowing or refusing the agent's request, with a scope choice. The sandbox policy and forbidden rules still bound what any grant can permit.".to_string(),
            "Exec-policy amendment, a separate and durable act: when approving a command, the user may also approve an amendment adding a prefix rule that permits future matching commands. That amendment is persisted both in memory and on disk, so it changes the control rule itself, not just one outcome. A turn- or session-scoped grant and a persisted amendment are different acts, and the adapter keeps them apart.".to_string(),
            "User interrupt of a running task: a session interrupt aborts the current task, if any.".to_string(),
        ]
    }

    fn termination(&self) -> String {
        "The loop terminates when the task is complete, when the user stops it, or when a sandbox or approval control refuses a required action.".to_string()
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
