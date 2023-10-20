use bevy::prelude::*;

use crate::game::direction::GridDirection;
use crate::game::model::*;
use crate::game::vector::GridVector;

use super::behaviors::Behavior;

#[derive(Component, Debug)]
pub struct MapEntity;

#[derive(Component, Debug, Default)]
pub struct GridPosition {
    pub coordinates: GridVector,
    pub direction: Option<GridDirection>,
}

#[derive(Component, Debug)]
pub struct Solid;

#[derive(Component, Debug, Eq, PartialEq, Copy, Clone)]
pub enum EntityType {
    Static,
    Character(Character),
    Door(Door),
    Switch(Switch),
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Cooldown(pub f32);

#[derive(Component, Debug, Eq, PartialEq, Copy, Clone)]
pub enum ControlledType {
    PlayerControlled,
    AIControlled,
}
