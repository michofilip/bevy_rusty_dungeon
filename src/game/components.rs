use bevy::prelude::*;

use crate::game::directions::GridDirection;
use crate::game::model::*;
use crate::game::vector::GridVector;

#[derive(Component, Debug)]
pub struct MapEntity;

#[derive(Component, Debug, Default)]
pub struct GridPosition {
    pub coordinates: GridVector,
    pub direction: Option<GridDirection>,
}

#[derive(Component, Debug)]
pub struct Solid;

#[derive(Component, Debug)]
pub struct Character {
    pub character_type: CharacterType,
}

impl Character {
    pub fn new(character_type: CharacterType) -> Self {
        Self { character_type }
    }
}

#[derive(Component, Debug)]
pub struct Door {
    // TODO use enum OPEN, CLOSE, LOCKED
    pub closed: bool,
}

impl Door {
    pub fn new(closed: bool) -> Self {
        Self { closed }
    }

    pub fn open(&mut self) {
        self.closed = false;
    }

    pub fn close(&mut self) {
        self.closed = true;
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Cooldown(pub f32);

#[derive(Component, Debug)]
pub struct PlayerControlled;
