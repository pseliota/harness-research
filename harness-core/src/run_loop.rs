pub trait Policy {
    fn choose(&self, observation: &str) -> String;
}

pub trait HarnessLoop {
    fn step(&mut self);

    fn run(&mut self);
}
