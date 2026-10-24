//! Adopting an existing harness on top of this core.
//!
//! This is the adapter shape for the monthly objective
//! of unifying a popular harness under the theory: an
//! existing harness is not rewritten, it is mapped onto
//! the core. Each method names one thing the adapter
//! must be able to say about the existing harness, in
//! the core's terms. Adapters themselves live in
//! the parallel `adaptations` crate, not here, so
//! the core stays free of any one harness: a later
//! adapter for a different harness implements the
//! same trait, so the two adoptions can be compared
//! in the core's terms.

use crate::membership::Candidate;

/// An existing harness, adopted on top of the core.
/// Implementing this trait for a real harness is the
/// adoption work; the trait itself only fixes what
/// "adopted" has to mean.
pub trait AdoptedHarness {
    /// The name of the existing harness, for identification only.
    fn name(&self) -> &str;

    /// The policy whose loop this harness runs.
    fn policy(&self) -> &str;

    /// Its loop, described in the core's terms.
    fn loop_description(&self) -> &str;

    /// Its action space at the current harness state, `A(h)`.
    fn action_space(&self) -> Vec<String>;

    /// Its state, split into world and harness parts.
    fn state_split(&self) -> (String, String);

    /// Its views: one observation function per principal.
    fn views(&self) -> Vec<String>;

    /// Its arbitration rule, stated explicitly — the rule itself,
    /// or that there is none, or that it is not yet stated —
    /// and its designer-fixed controls. Silence about
    /// arbitration is not a statement: a reader must be able
    /// to tell absence from an unnamed rule.
    fn controls(&self) -> Vec<String>;

    /// Its termination: when a run terminates, and who decides.
    /// "Ending" is reserved for retiring the harness itself;
    /// a run terminates, a harness ends.
    fn termination(&self) -> String;

    /// The membership test, applied to this harness.
    fn membership(&self) -> Candidate;
}
