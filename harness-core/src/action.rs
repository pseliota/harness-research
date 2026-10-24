//! The action space at levels of abstraction, not a single set.
//!
//! Theory: "Action space at levels of abstraction".
//! There is no single set `A` for every level. At the
//! lowest level an action is a token; at the next level
//! a parsed move; moves execute into effects, and some
//! executions contain whole inner loops. The available
//! moves depend on harness state: `A(h)`, not `A`.
//! This is a stub.

/// A token at the lowest level: an element of the vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token(pub u32);

/// A parsed move: one level up from tokens.
/// A move is a quotient of token sequences, not a subset
/// of them: many strings map to the same move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedMove {
    /// Emit text.
    EmitText(String),
    /// Call a named tool with arguments.
    CallTool { name: String, arguments: String },
    /// Spawn a subagent; its execution is a whole inner loop.
    SpawnSubagent(String),
    /// Stop.
    Stop,
}

/// A parser: the function that maps token strings to moves.
/// A stub interface: no parsing is implemented.
pub trait Parser {
    fn parse(&self, tokens: &[Token]) -> Option<ParsedMove>;
}

/// The action space at a harness state: `A(h)`, not `A`.
/// Stub only: listing available moves is not implemented.
pub struct ActionSpace;

impl ActionSpace {
    /// The moves the harness will attempt at this harness state.
    /// The argument is `h` alone, not `w` and not the whole
    /// state: an attempt is a decision of the harness, and
    /// success still depends on `w`. Attempts, not guarantees:
    /// whether a move succeeds can still depend on the world.
    /// The type of `h` is left generic on purpose: the theory
    /// does not fix what `h` contains.
    pub fn available_moves<H>(&self, _harness_state: &H) -> Vec<ParsedMove> {
        todo!()
    }
}
