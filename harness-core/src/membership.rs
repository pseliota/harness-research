//! Membership: what counts as a harness, and what does not.
//!
//! Theory: "Membership". A candidate is an agent harness
//! exactly when four things hold: it runs a loop; its
//! machinery is designer-controlled and outside the
//! policy's write set except for a permitted write scope
//! scope; the policy can act through it; and at least
//! one designer-fixed control can alter or suspend
//! execution without the policy's cooperation.
//! Weakness is not absence: a bad harness is still a
//! harness. This is a stub.

/// A candidate system, described only by the four
/// membership conditions. How each condition is checked
/// for a real system is left to an adapter (see
/// `adoption`); this struct is the shape of the test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate {
    /// It runs a loop for one policy on one task, where
    /// the next step can depend on the previous observation.
    pub runs_loop: bool,
    /// Its machinery is designer-controlled, outside the
    /// policy's write set except for permitted write scope.
    pub designer_controlled: bool,
    /// The policy can act through it: at least one action
    /// lands in the world or in harness state.
    pub policy_can_act: bool,
    /// At least one designer-fixed control can alter or
    /// suspend execution without the policy's cooperation.
    pub has_designer_control: bool,
}

/// Apply the membership test. Stub only: the four
/// conditions are the test, their conjunction is not
/// implemented here, so that no hidden logic can smuggle
/// in a fifth condition.
pub fn is_harness(_candidate: &Candidate) -> bool {
    todo!()
}
