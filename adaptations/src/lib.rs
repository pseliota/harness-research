//! Stub adapters for popular harnesses, on top of `harness_core`.
//!
//! Nothing here runs a harness. Each module implements
//! `harness_core::adoption::AdoptedHarness` for one
//! existing harness, naming in the core's terms what an
//! adapter would have to say about it. The returned
//! strings are descriptions, not behaviour, and each
//! adapter lists gaps to be verified against the real
//! harness before its mapping is treated as a claim.

pub mod claude_code;
pub mod codex;
pub mod copilot;
pub mod cursor;
pub mod gemini_cli;
pub mod opencode;
pub mod swe_agent;
