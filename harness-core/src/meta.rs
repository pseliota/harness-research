//! The meta-rule.
//!
//! Theory: "Thresholds without a designer" and "The
//! minimal core". Authority predates the act it
//! authorises and sits outside the party it judges.
//! Revision is a recorded, prospective, visible design
//! act. Where no rule covers the case, inherited values
//! freeze rather than relax. This is a stub.

/// A design act: a revision to the core, recorded with
/// provenance and visible to every principal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignAct {
    pub description: String,
    /// Design acts are prospective only: they apply to
    /// future traces, and do not erase prior verdicts.
    pub prospective: bool,
}

/// A meta-rule: how a threshold value is computed, from
/// what inputs, within what bounds. The quantity being
/// judged must not set its own bar.
/// A stub interface: computation is not implemented.
pub trait MetaRule {
    fn threshold(&self) -> u64;
}

/// Record a design act. Stub only.
pub fn record_design_act(_act: &DesignAct) {
    todo!()
}

/// Freeze inherited values where no meta-rule covers
/// the case, rather than relaxing them to escape a
/// defect. Stub only.
pub fn freeze_if_uncovered() {
    todo!()
}
