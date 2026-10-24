#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate {
    pub runs_loop: bool,
    pub designer_controlled: bool,
    pub policy_can_act: bool,
    pub has_designer_control: bool,
}

pub fn is_harness(_candidate: &Candidate) -> bool {
    todo!()
}
