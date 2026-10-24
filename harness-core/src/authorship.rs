//! Agent authorship over harness state.
//!
//! Theory: the agent may write harness state within a
//! scope the designer permitted, and thereby shape its
//! own future observations and transitions. The fixed
//! core and the modification policy stay outside the
//! agent's write set. This is a stub.

/// The fixed core: immutable base, runtime that applies
/// edits, and versioning that records provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedCore {
    pub description: String,
}

/// The mutable supplement: notes, memories, skills, and
/// subagent specifications the agent writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutableSupplement {
    pub description: String,
}

/// A modification policy, fixed by the designer: what may
/// be edited, when, on what evidence, with what provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModificationPolicy {
    pub description: String,
}

impl ModificationPolicy {
    /// Whether a proposed agent write is within the permitted scope.
    /// Stub only.
    pub fn permits(&self, _write: &str) -> bool {
        todo!()
    }

    /// Apply a permitted agent write to harness state.
    /// Stub only.
    pub fn apply(&self, _write: &str) {
        todo!()
    }
}
