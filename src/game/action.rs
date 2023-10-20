use bevy::prelude::*;

use super::{components::GridPosition, direction::GridDirection, vector::GridVector};

#[derive(Clone, Copy)]
pub enum Action {
    SetCoordinates(Entity, GridVector),
    SetDirection(Entity, GridDirection),
}

impl Action {
    pub fn execute(&self, world: &mut World) -> Result<Vec<Action>, ()> {
        match self {
            Action::SetCoordinates(entity, coordinates) => {
                set_coordinates(*entity, coordinates, world)
            }
            Action::SetDirection(entity, direction) => set_direction(*entity, direction, world),
        }
    }
}

fn set_coordinates(
    entity: Entity,
    coordinates: &GridVector,
    world: &mut World,
) -> Result<Vec<Action>, ()> {
    let Some(mut grid_position) = world.get_mut::<GridPosition>(entity) else {
        return Err(());
    };

    grid_position.coordinates = *coordinates;

    Ok(Vec::new())
}

fn set_direction(
    entity: Entity,
    direction: &GridDirection,
    world: &mut World,
) -> Result<Vec<Action>, ()> {
    let Some(mut grid_position) = world.get_mut::<GridPosition>(entity) else {
        return Err(());
    };

    if grid_position.direction.is_some() {
        grid_position.direction = Some(*direction);
    }

    Ok(Vec::new())
}
