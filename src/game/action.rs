use bevy::prelude::*;

use super::{components::GridPosition, direction::GridDirection, vector::GridVector};

#[derive(Clone, Copy)]
pub enum Action {
    MoveAction(Entity, GridDirection),
}

impl Action {
    pub fn execute(&self, world: &mut World) -> Result<Vec<Action>, ()> {
        match self {
            Action::MoveAction(entity, direction) => execute_move_action(*entity, direction, world),
        }
    }
}

fn execute_move_action(
    entity: Entity,
    direction: &GridDirection,
    world: &mut World,
) -> Result<Vec<Action>, ()> {
    let Some(mut grid_position) = world.get_mut::<GridPosition>(entity) else {
        return Err(());
    };

    grid_position.coordinates += GridVector::from_direction(direction);

    Ok(Vec::new())
}
