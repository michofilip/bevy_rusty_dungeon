use super::action::Action;

pub mod selectors;

#[derive(Clone)]
pub struct Behavior {
    pub cooldown: f32,
    pub actions: Vec<Action>,
}

impl Behavior {
    pub fn new(cooldown: f32, actions: Vec<Action>) -> Self {
        Self { cooldown, actions }
    }

    pub fn empty(cooldown: f32) -> Self {
        Self::new(cooldown, Vec::new())
    }
}
