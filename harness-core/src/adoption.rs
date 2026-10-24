//! Adopting an existing harness on top of this core.
//!
//! This is the adapter shape for the monthly objective
//! of unifying a popular harness under the theory: an
//! existing harness is not rewritten, it is mapped onto
//! the core. Each method names one thing the adapter
//! must be able to say about the existing harness, in
//! the core's terms. This is a stub interface: no
//! existing harness is adapted here.

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

    /// Its arbitration rule, if any, and its designer-fixed controls.
    fn controls(&self) -> Vec<String>;

    /// Its ending: when a run ends, and who decides.
    fn ending(&self) -> String;

    /// The membership test, applied to this harness.
    fn membership(&self) -> Candidate;
}
