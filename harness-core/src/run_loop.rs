//! The policy loop itself.
//!
//! Theory: "Definitions" and "The loop". An agent is a policy
//! `pi(a | o)`; the harness carries out the transition
//! `p(s' | s, a)` and the observation `p(o | s)`.
//! This is a stub: no loop is actually run.

/// A policy: chooses an action from an observation.
/// A stub interface: choosing is not implemented.
pub trait Policy {
    fn choose(&self, observation: &str) -> String;
}

/// A harness loop: carries successive observations and
/// transitions for one policy on one task.
/// A stub interface: stepping and running are not implemented.
pub trait HarnessLoop {
    /// One step: observe, choose, transition, observe again.
    fn step(&mut self);

    /// Run until the loop's stop rule fires.
    fn run(&mut self);
}
