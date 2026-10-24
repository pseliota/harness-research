//! Rough stubs for the harness theory in `theory.md`.
//!
//! No behaviour is implemented here, and there are no tests.
//! Each module names the shape of one part of the theory;
//! stub functions use `todo!()` and stub traits have no
//! method bodies. The point of this package, for now, is
//! to make the theory's structure checkable against code:
//! if a claim in the theory has no counterpart here, or
//! a stub here has no counterpart in the theory, the two
//! have drifted apart.

pub mod action;
pub mod adoption;
pub mod arbitration;
pub mod authorship;
pub mod boundary;
pub mod extensions;
pub mod liveness;
pub mod membership;
pub mod meta;
pub mod run_loop;
pub mod state;
pub mod views;
