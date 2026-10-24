use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

pub struct GeminiCli;

impl GeminiCli {
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
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

    fn observation_functions(&self) -> Vec<String> {
        vec![
            "Agent: the context history, including tool results.".to_string(),
            "User: terminal output, diffs, and summaries.".to_string(),
            "Overseer: run transcript and cost records, where recorded.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Precedence: none — no designer-fixed precedence rule deciding between contending principals' proposals was found. What exists is veto and approval over a single proposed move, not a choice between proposals. The policy engine decides one move at a time: ALLOW, DENY, or ASK_USER, from the first matching rule when rules are sorted by priority, with tiers admin (5), user (4), workspace (3), extension (2), default (1), so an admin rule beats a user rule about the same move — rule precedence, not a precedence rule between proposals. DENY blocks the move with a policy-violation error; ASK_USER sends it to the user for confirmation. Confirmation itself is raced between the terminal and the IDE companion, first response winning, which is one user through two channels, not two principals. Hooks aggregate the same way: any blocking hook denies, and an ask decision stands only if no hook blocks. No rule suspending only a contested transition while unrelated work proceeds was found.".to_string(),
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
