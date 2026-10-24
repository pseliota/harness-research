//! Observation functions, one per principal.
//!
//! Theory: "Observation functions and precedence". One state,
//! one observation function per principal, each satisfying
//! sufficiency and separation: enough to do the job, never
//! conferring a write by showing a read. This is a stub.

/// A principal: a party with its own observation function,
/// its own write set, and its own goal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Principal {
    Agent,
    User,
    Overseer,
}

/// An observation: what one principal is shown of the state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    pub content: String,
}

/// An observation function: the harness's function for one principal.
/// A stub interface: rendering is not implemented.
pub trait ObservationFunction {
    /// The observation for one principal of one state:
    /// `p_i(o_i | s)`, where `s` is the whole state.
    /// The argument is `s`, not a pre-rendered view of it:
    /// what a principal is shown is a function of the state.
    /// The type of `s` is left generic on purpose: the theory
    /// does not fix what `s` contains.
    fn observe<S>(&self, principal: Principal, state: &S) -> Observation;

    /// Sufficiency: does the observation function give this principal enough
    /// to do its job? Stub only.
    fn is_sufficient(&self, _principal: Principal) -> bool {
        todo!()
    }

    /// Separation: does seeing something confer writing it?
    /// An observation function must not. Stub only.
    fn separates_read_from_write(&self, _principal: Principal) -> bool {
        todo!()
    }
}
