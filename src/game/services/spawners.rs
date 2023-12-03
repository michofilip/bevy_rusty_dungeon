use bevy::prelude::*;

use crate::game::components::*;
use crate::game::directions::GridDirection;
use crate::game::model::CharacterType;
use crate::game::vector::GridVector;

pub fn spawn_floor(vec: GridVector, world: &mut World) -> Entity {
    world
        .spawn((
            Name::new("floor"),
            GridPosition {
                coordinates: vec,
                direction: None,
            },
            MapEntity,
        ))
        .id()
}

pub fn spawn_wall(vec: GridVector, world: &mut World) -> Entity {
    world
        .spawn((
            Name::new("wall"),
            GridPosition {
                coordinates: vec,
                direction: None,
            },
            Solid,
            MapEntity,
        ))
        .id()
}

pub fn spawn_door(vec: GridVector, closed: bool, world: &mut World) -> Entity {
    world
        .spawn((
            Name::new("door"),
            Door::new(closed),
            GridPosition {
                coordinates: vec,
                direction: None,
            },
            Solid,
            MapEntity,
        ))
        .id()
}

pub fn spawn_player(vec: GridVector, cooldown: f32, world: &mut World) -> Entity {
    world
        .spawn((
            Name::new("player"),
            Character::new(CharacterType::Player),
            GridPosition {
                coordinates: vec,
                direction: Some(GridDirection::North),
            },
            Solid,
            MapEntity,
            Cooldown(cooldown),
            PlayerControlled,
        ))
        .id()
}

pub fn spawn_monster(vec: GridVector, cooldown: f32, world: &mut World) -> Entity {
    world
        .spawn((
            Name::new("monster"),
            Character::new(CharacterType::Monster),
            GridPosition {
                coordinates: vec,
                direction: Some(GridDirection::North),
            },
            Solid,
            MapEntity,
            Cooldown(cooldown),
        ))
        .id()
}
