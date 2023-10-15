use bevy::prelude::*;

use super::{action::Action, direction::GridDirection};

#[derive(Clone, Copy)]
pub enum Behavior {
    Wait,
    Move(GridDirection),
}

impl Behavior {
    pub fn cooldown(&self, world: &mut World) -> f32 {
        match self {
            Behavior::Wait => 1.0,
            Behavior::Move(_) => 1.0,
        }
    }

    pub fn actions(&self, entity: Entity) -> Vec<Action> {
        match self {
            Behavior::Wait => Vec::new(),
            Behavior::Move(direction) => vec![Action::MoveAction(entity, direction.clone())],
        }
    }
}
