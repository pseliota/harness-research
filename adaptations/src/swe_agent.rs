use harness_core::adoption::AdoptedHarness;
use harness_core::membership::Candidate;

pub struct SweAgent;

impl SweAgent {
    pub fn gaps(&self) -> Vec<&'static str> {
        vec![
            "No live view for a second principal: an overseer gets the trajectory file and the trajectory inspector after the run, not an observation function of its own during it.",
            "No precedence rule: there is a single policy, so the only conflicts are vetoes, a blocklist refusing a command, not principals contending over a transition.",
            "No human intervention in a run already under way: a human model replaces the policy rather than pausing, editing, redirecting, or taking over a running loop.",
            "Control is veto plus cost, not admission: the blocklist refuses known bad commands, it does not bound in advance what a shell string can do.",
            "Termination and verdict come apart: a run can terminate as submitted without ever having succeeded, and success is judged outside the loop by the SWE-bench evaluation hook.",
        ]
    }
}

impl AdoptedHarness for SweAgent {
    fn name(&self) -> &str {
        "SWE-agent"
    }

    fn policy(&self) -> &str {
        "The model named in the configuration, queried on a history passed through the configured history processors."
    }

    fn loop_description(&self) -> &str {
        "DefaultAgent.run: while not step_output.done, step: query the model, parse a thought and an action, execute the action in a container environment, append the observation, and save the trajectory."
    }

    fn action_space(&self) -> Vec<String> {
        vec![
            "bash: any shell string, minus the blocklist, minus the remaining budget and time (an open move kind, not a closed list)".to_string(),
            "registry bundle: the configured tool registry".to_string(),
            "editor bundle: the configured file-editing tools".to_string(),
            "review-on-submit bundle: a designer-fixed check that runs on submit".to_string(),
        ]
    }

    fn state_split(&self) -> (String, String) {
        (
            "World: the repository files and their test outcomes, as the object of the task, held in a designer-owned container sandbox.".to_string(),
            "Harness: the trajectory and history lists, the system and instance templates, the tool bundles, the history processors, and the cost, timeout, and blocklist configuration.".to_string(),
        )
    }

    fn observation_functions(&self) -> Vec<String> {
        vec![
            "Agent: the next-step observation, built from the trajectory by the configured history processor, rendered as OBSERVATION: {{observation}} and truncated at 100,000 characters with the elided count noted.".to_string(),
        ]
    }

    fn controls(&self) -> Vec<String> {
        vec![
            "Precedence: none — there is a single policy, so the only conflicts are vetoes, a blocklist refusing a command, not principals contending over a transition.".to_string(),
            "Per-instance and total cost limits: exceeding either ends the run.".to_string(),
            "Command timeout of 30 seconds, total execution timeout of 1800 seconds, and exit after repeated consecutive timeouts.".to_string(),
            "Blocklist: interactive and escape commands are refused and not executed; the model is requieried, up to three times, for a different action.".to_string(),
            "Context window: exceeding it ends the run.".to_string(),
            "Designer-fixed history processors rewrite history between steps: the classic LastNObservations replaces older observations with a note that lines were omitted.".to_string(),
        ]
    }

    fn termination(&self) -> String {
        "The loop terminates, done becomes true, on submit, on forfeit, on cost, context, or timeout, or by autosubmission after an error, where the harness submits the diff from the last trajectory step's state. On submit the review-on-submit bundle shows the model its own diff and asks it to rerun the reproduction script, remove that script, revert any test files it touched, and submit again. Success is judged outside the loop by the SWE-bench evaluation hook, not by the loop itself.".to_string()
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
