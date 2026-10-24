//! Applications of the core, not the core itself.
//!
//! Theory: succession, ending, debts, chains, identity,
//! trust, anchors, and root rotation are applications of
//! the meta-rule to the next thing that could evade it.
//! They are kept here, outside the core modules, so that
//! building on the core does not require adopting them.
//! Everything in this module is a stub.

/// Succession: transfer of the designer role itself,
/// legitimate only by a core rule fixed before the absence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succession {
    pub description: String,
}

impl Succession {
    /// Whether a successor is legitimate under a prior rule.
    /// Stub only.
    pub fn is_legitimate(&self) -> bool {
        todo!()
    }
}

/// Ending: retirement of the standing machinery itself,
/// as a design act, not a verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ending {
    pub description: String,
}

impl Ending {
    /// Announce, drain, and archive an ending. Stub only.
    pub fn end(&self) {
        todo!()
    }
}

/// An outstanding obligation that passes with the office.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Debt {
    pub description: String,
}

/// Identity across revision: the traces, not the code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub description: String,
}

impl Identity {
    /// Whether authority, revision, and debt traces are intact.
    /// Stub only.
    pub fn traces_intact(&self) -> bool {
        todo!()
    }
}

/// A record with provenance, chained and anchored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRecord {
    pub description: String,
}

/// A ledger of trust roots, rotated by prior rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootLedger {
    pub description: String,
}

impl RootLedger {
    /// Walk from a held root to the present root, checking
    /// each vouch. Stub only.
    pub fn walk_from(&self, _held_root: &str) -> bool {
        todo!()
    }
}
