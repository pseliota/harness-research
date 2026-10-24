use crate::membership::Candidate;

pub trait AdoptedHarness {
    fn name(&self) -> &str;

    fn policy(&self) -> &str;

    fn loop_description(&self) -> &str;

    fn action_space(&self) -> Vec<String>;

    fn state_split(&self) -> (String, String);

    fn observation_functions(&self) -> Vec<String>;

    fn controls(&self) -> Vec<String>;

    fn termination(&self) -> String;

    fn membership(&self) -> Candidate;
}
