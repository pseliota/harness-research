#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Agent,
    Harness,
    World,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    pub requires_agent_change: bool,
    pub requires_world_change: bool,
}

pub fn classify(_change: &Change) -> Side {
    todo!()
}
