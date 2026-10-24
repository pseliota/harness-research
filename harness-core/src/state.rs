#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State<W, H> {
    pub world: W,
    pub harness: H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectTarget {
    World,
    Harness,
    Both,
}

impl<W, H> State<W, H> {
    pub fn effect_target(&self, _action: &str) -> EffectTarget {
        todo!()
    }
}
