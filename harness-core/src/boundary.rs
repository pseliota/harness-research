//! The harness boundary, by designer control.
//!
//! Theory: "The harness boundary: what the designer controls".
//! The environment splits three ways by who can change what.
//! This is a stub: classification is not implemented.

/// Which side of the boundary a thing sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// What the agent controls: its policy, not its prompt.
    Agent,
    /// What the designer controls without changing the agent or the world.
    Harness,
    /// What neither designer nor agent controls.
    World,
}

/// A proposed change, described only by what changing it would require.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    /// Changing it would mean retraining or replacing the agent.
    pub requires_agent_change: bool,
    /// Changing it would mean changing the world.
    pub requires_world_change: bool,
}

/// The designer-control test: can the designer change this
/// arbitrarily while the agent and the world stay fixed?
/// Stub only.
pub fn classify(_change: &Change) -> Side {
    todo!()
}
