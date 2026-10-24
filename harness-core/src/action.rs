#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedMove {
    EmitText(String),
    CallTool { name: String, arguments: String },
    SpawnSubagent(String),
    Stop,
}

pub trait Parser {
    fn parse(&self, tokens: &[Token]) -> Option<ParsedMove>;
}

pub struct ActionSpace;

impl ActionSpace {
    pub fn available_moves<H>(&self, _harness_state: &H) -> Vec<ParsedMove> {
        todo!()
    }
}
