//! Stub adapters for open-source harnesses, on top of `harness_core`.
//!
//! Only harnesses whose agent code is under a permissive
//! open-source licence belong here. A product that is
//! only partly open counts only for its open part:
//! Codex, for example, is represented by its Apache 2.0
//! CLI, not by its IDE extension or cloud service.
//!
//! Nothing here runs a harness. Each module implements
//! `harness_core::adoption::AdoptedHarness` for one
//! existing harness, naming in the core's terms what an
//! adapter would have to say about it. The returned
//! strings are descriptions, not behaviour, and each
//! adapter lists gaps to be verified against the real
//! harness before its mapping is treated as a claim.

pub mod codex_cli;
pub mod gemini_cli;
pub mod opencode;
pub mod swe_agent;
