#[derive(Debug, Eq, PartialEq)]
pub enum CharacterType {
    Player,
    Monster,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Switch {
    pub on: bool,
}

impl Switch {
    pub fn new(on: bool) -> Self {
        Self { on }
    }

    pub fn switch(&mut self) {
        self.on = !self.on;
    }
}
