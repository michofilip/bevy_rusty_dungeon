#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Character {
    pub character_type: CharacterType,
}

impl Character {
    pub fn new(character_type: CharacterType) -> Self {
        Self { character_type }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CharacterType {
    Player,
    Monster,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Door {
    pub close: bool,
}

impl Door {
    pub fn new(close: bool) -> Self {
        Self { close }
    }

    pub fn open(&self) -> Self {
        Self::new(false)
    }

    pub fn close(&self) -> Self {
        Self::new(true)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Switch {
    pub on: bool,
}

impl Switch {
    pub fn new(on: bool) -> Self {
        Self { on }
    }

    pub fn switch(&self) -> Self {
        Self::new(!self.on)
    }
}
