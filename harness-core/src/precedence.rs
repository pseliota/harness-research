#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub proposer: String,
    pub transition: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecedenceRule {
    pub description: String,
}

impl PrecedenceRule {
    pub fn enabled(&self, _proposals: &[Proposal]) -> Vec<Proposal> {
        todo!()
    }

    pub fn suspend_contested(&self, _proposals: &[Proposal]) -> Vec<Proposal> {
        todo!()
    }

    pub fn shows_decision_to(&self, _preempted: &Proposal) -> String {
        todo!()
    }
}
