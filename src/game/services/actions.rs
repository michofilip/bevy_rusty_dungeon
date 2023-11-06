use bevy::prelude::*;

use crate::game::components::{Cooldown, GridPosition};
use crate::game::directions::GridDirection;
use crate::game::vector::GridVector;

pub enum MoveType {
    Walk,
    Run,
}

pub fn wait(entity: Entity, world: &mut World) {
    update_cooldown(entity, 1.0, world);
}

pub fn move_to(
    entity: Entity,
    coordinates: GridVector,
    direction: GridDirection,
    move_type: MoveType,
    world: &mut World,
) {
    let cooldown = match move_type {
        MoveType::Walk => 1.0,
        MoveType::Run => 0.5,
    };

    let Some(mut grid_position) = world.get_mut::<GridPosition>(entity) else {
        return;
    };

    grid_position.coordinates = coordinates;
    if grid_position.direction.is_some() {
        grid_position.direction = Some(direction);
    }

    update_cooldown(entity, cooldown, world);
}

pub fn attack(source_entity: Entity, target_entity: Entity, world: &mut World) {
    // TODO
    world.despawn(target_entity);
    update_cooldown(source_entity, 1.0, world);
}

fn update_cooldown(entity: Entity, cooldown: f32, world: &mut World) {
    world.get_mut::<Cooldown>(entity).unwrap().0 += cooldown;
}
