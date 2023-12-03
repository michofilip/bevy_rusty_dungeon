use bevy::{prelude::*, utils::HashSet};

use crate::game::components::*;
use crate::game::directions::GridDirection;
use crate::game::model::CharacterType;
use crate::game::vector::GridVector;

pub fn get_shifted_coordinates(
    entity: Entity,
    direction: GridDirection,
    world: &mut World,
) -> Option<GridVector> {
    world
        .get::<GridPosition>(entity)
        .map(|position| position.coordinates + GridVector::from_direction(&direction))
}

pub fn get_solids(world: &mut World) -> HashSet<GridVector> {
    let mut query = world.query_filtered::<&GridPosition, With<Solid>>();
    query
        .iter(world)
        .map(|position| position.coordinates)
        .collect::<HashSet<GridVector>>()
}

pub fn get_static_solids(world: &mut World) -> HashSet<GridVector> {
    let mut query =
        world.query_filtered::<&GridPosition, (With<Solid>, Without<Character>, Without<Door>)>();
    query
        .iter(world)
        .map(|position| position.coordinates)
        .collect::<HashSet<GridVector>>()
}

pub fn is_solids_at(coordinates: GridVector, world: &mut World) -> bool {
    let mut query = world.query_filtered::<&GridPosition, With<Solid>>();
    query
        .iter(world)
        .filter(|position| position.coordinates == coordinates)
        .next()
        .is_some()
}

pub fn get_character_at(
    character_type: CharacterType,
    coordinates: GridVector,
    world: &mut World,
) -> Option<Entity> {
    let mut query = world.query::<(Entity, &GridPosition, &Character)>();
    query
        .iter(world)
        .filter(|(_, position, character)| {
            position.coordinates == coordinates && character.character_type == character_type
        })
        .map(|(entity, _, _)| entity)
        .next()
}

pub fn get_door_at(coordinates: GridVector, world: &mut World) -> Option<Entity> {
    let mut query = world.query_filtered::<(Entity, &GridPosition), With<Door>>();
    query
        .iter(world)
        .filter(|(_, position)| position.coordinates == coordinates)
        .map(|(entity, _)| entity)
        .next()
}
