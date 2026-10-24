#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Principal {
    Agent,
    User,
    Overseer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    pub content: String,
}

pub trait ObservationFunction {
    fn observe<S>(&self, principal: Principal, state: &S) -> Observation;

    fn is_sufficient(&self, _principal: Principal) -> bool {
        todo!()
    }

    fn separates_read_from_write(&self, _principal: Principal) -> bool {
        todo!()
    }
}
