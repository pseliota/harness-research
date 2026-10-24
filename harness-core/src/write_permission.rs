#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedCore {
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutableSupplement {
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModificationPolicy {
    pub description: String,
}

impl ModificationPolicy {
    pub fn permits(&self, _write: &str) -> bool {
        todo!()
    }

    pub fn apply(&self, _write: &str) {
        todo!()
    }
}
