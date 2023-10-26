use bevy::prelude::*;

use crate::game::components::*;
use crate::game::directions::GridDirection;
use crate::game::model::Character;
use crate::game::model::CharacterType;
use crate::game::vector::GridVector;

pub fn spawn_level(mut commands: Commands) {
    for y in -10..=10 {
        for x in -10..=10 {
            let vec = GridVector::new(x, y);
            spawn_floor(vec, &mut commands);

            if x == -10 || y == -10 || x == 10 || y == 10 {
                spawn_wall(vec, &mut commands);
            }
        }
    }

    spawn_player(GridVector::new(0, 0), 0.0, &mut commands);
    spawn_monster(GridVector::new(5, 5), 0.0, &mut commands);
    spawn_monster(GridVector::new(5, -5), 0.0, &mut commands);
    spawn_monster(GridVector::new(-5, 5), 0.0, &mut commands);
    spawn_monster(GridVector::new(-5, -5), 0.0, &mut commands);
}

fn spawn_floor(vec: GridVector, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("floor"),
            EntityType::Static,
            GridPosition {
                coordinates: vec,
                direction: None,
            },
            MapEntity,
        ))
        .id()
}

fn spawn_wall(vec: GridVector, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("wall"),
            EntityType::Static,
            GridPosition {
                coordinates: vec,
                direction: None,
            },
            Solid,
            MapEntity,
        ))
        .id()
}

fn spawn_player(vec: GridVector, cooldown: f32, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("player"),
            EntityType::Character(Character::new(CharacterType::Player)),
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

fn spawn_monster(vec: GridVector, cooldown: f32, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("monster"),
            EntityType::Character(Character::new(CharacterType::Monster)),
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
