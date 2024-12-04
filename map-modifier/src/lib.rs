use homm5_types::{quest::Quest, Homm5Type};

pub mod quest;

pub trait GenerateBoilerplate {
    type Output: Homm5Type;

    fn generate(&self) -> Self::Output;
}

pub struct ModifiersQueue {
    quest_queue: Vec<Box<dyn GenerateBoilerplate<Output = Quest>>>
}