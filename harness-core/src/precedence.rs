//! Conflict between principals: precedence is harness, not a principal.
//!
//! Theory: "Observation functions and precedence". A precedence rule is a
//! designer-fixed rule on transitions. It is veto, not compulsion: it disables
//! some proposed transitions and forces none. Preemption
//! must be legible to the preempted principal, and a
//! conflict no precedence decides suspends only the
//! contested transition. This is a stub.

/// A transition proposed by a principal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub proposer: String,
    pub transition: String,
}

/// A designer-fixed precedence rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecedenceRule {
    pub description: String,
}

impl PrecedenceRule {
    /// Select which proposed transitions are enabled.
    /// The rest are disabled for this step. Nothing is
    /// invented: a proposal that is not enabled is not
    /// replaced by a compromise the rule made up.
    /// Stub only.
    pub fn enabled(&self, _proposals: &[Proposal]) -> Vec<Proposal> {
        todo!()
    }

    /// Suspend only the contested transition when no
    /// precedence decides between conflicting proposals,
    /// and surface the conflict. Stub only.
    pub fn suspend_contested(&self, _proposals: &[Proposal]) -> Vec<Proposal> {
        todo!()
    }
}
