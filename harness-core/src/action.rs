use crate::state::EffectTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedMove {
    EmitText(String),
    CallTool { name: String, arguments: String },
    SpawnSubagent(String),
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub intent: ParsedMove,
    pub target: EffectTarget,
}

pub trait Parser {
    fn parse(&self, tokens: &[Token]) -> Option<ParsedMove>;
}

pub struct ActionSpace;

impl ActionSpace {
    pub fn available_moves<H>(&self, _harness_state: &H) -> Vec<Move> {
        todo!()
    }
}
