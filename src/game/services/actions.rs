use bevy::prelude::*;

use crate::game::components::*;
use crate::game::directions::GridDirection;
use crate::game::model::CharacterType;
use crate::game::services::utils;
use crate::game::vector::GridVector;

pub enum MoveType {
    Walk,
    Run,
}

pub fn wait(entity: Entity, world: &mut World) -> bool {
    update_cooldown(entity, 1.0, world);
    true
}

pub fn attempt_to_move(
    entity: Entity,
    coordinates: GridVector,
    direction: GridDirection,
    move_type: MoveType,
    world: &mut World,
) -> bool {
    if utils::is_solids_at(coordinates, world) {
        return false;
    }

    let mut position = world.get_mut::<GridPosition>(entity).unwrap();
    position.coordinates = coordinates;
    if position.direction.is_some() {
        position.direction = Some(direction);
    }

    let cooldown = match move_type {
        MoveType::Walk => 1.0,
        MoveType::Run => 0.5,
    };
    update_cooldown(entity, cooldown, world);

    true
}

pub fn attempt_to_attack(
    entity: Entity,
    coordinates: GridVector,
    character_type: CharacterType,
    world: &mut World,
) -> bool {
    let Some(target_entity) = utils::get_character_at(character_type, coordinates, world) else {
        return false;
    };

    // TODO
    world.despawn(target_entity);
    update_cooldown(entity, 1.0, world);

    true
}

pub fn attempt_to_open_door(entity: Entity, coordinates: GridVector, world: &mut World) -> bool {
    let Some(door_entity) = utils::get_door_at(coordinates, world) else {
        return false;
    };

    let mut door = world.get_mut::<Door>(door_entity).unwrap();
    if door.closed {
        door.open();
        world.entity_mut(door_entity).remove::<Solid>();
        update_cooldown(entity, 1.0, world);
        true
    } else {
        false
    }
}

fn update_cooldown(entity: Entity, cooldown: f32, world: &mut World) {
    world.get_mut::<Cooldown>(entity).unwrap().0 += cooldown;
}
