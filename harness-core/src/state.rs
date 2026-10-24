//! The state split: `s = (w, h)`.
//!
//! Theory: "Definitions" and "The minimal core". World state is what neither
//! designer nor agent controls; harness state is what the
//! designer controls. This is a stub.

/// State as a pair: world state and harness state.
/// The concrete types are left generic on purpose: the
/// theory does not fix what `w` or `h` contain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State<W, H> {
    pub world: W,
    pub harness: H,
}

/// Which state component an action changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectTarget {
    /// The action changes world state.
    World,
    /// The action changes harness state.
    Harness,
    /// The action changes both.
    Both,
}

impl<W, H> State<W, H> {
    /// Sort an action by where its effect lands.
    /// Stub only: the action type is left as a name here.
    pub fn effect_target(&self, _action: &str) -> EffectTarget {
        todo!()
    }
}
