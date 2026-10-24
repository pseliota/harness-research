//! Liveness over histories, and progress toward a vague or shifting goal.
//!
//! Theory: "Liveness under precedence" and "Progress
//! toward a vague or shifting G". Safety is step-local;
//! liveness is judged over traces, by counters and timers
//! against designer-fixed thresholds, with progress
//! measured against aspiration, tied proxies, and recorded
//! epochs. This is a stub.

/// A trace: the history of a run, as the core sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    pub steps: Vec<String>,
}

/// A counter or timer verdict over a trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    /// The run is held, but not yet defective.
    Held,
    /// A counter or timer crossed its threshold.
    Defective,
    /// An epoch completed under its aspiration.
    Complete,
}

/// An aspiration level: good enough, on criteria the
/// principal can judge. Not an optimum, and not a point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aspiration {
    pub description: String,
}

/// An epoch: one goal revision, with recorded provenance.
/// Counters and timers run within an epoch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Epoch {
    pub goal: String,
    pub aspiration: Aspiration,
}

/// Judge a trace against designer-fixed thresholds.
/// Stub only: no counting or timing is implemented.
pub fn judge(_trace: &Trace, _threshold: u64) -> RunStatus {
    todo!()
}

/// Whether a trace counts as progress toward an aspiration.
/// A proxy counts only if tied to the aspiration; gain
/// about matters irrelevant to the goal is not progress.
/// Stub only.
pub fn is_progress(_trace: &Trace, _aspiration: &Aspiration) -> bool {
    todo!()
}
