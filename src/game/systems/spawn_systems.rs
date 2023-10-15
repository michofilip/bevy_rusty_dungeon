use bevy::prelude::*;

use crate::game::components::*;
use crate::game::direction::GridDirection;
use crate::game::model::Character;
use crate::game::model::CharacterType;
use crate::game::resources::Level;
use crate::game::states::GameState;
use crate::game::vector::GridVector;

pub fn spawn_level(
    mut commands: Commands,
    mut level: ResMut<Level>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for y in -10..=10 {
        for x in -10..=10 {
            let vec = GridVector::new(x, y);
            let floor_entity = spawn_floor(vec, &mut commands);
            level.insert(vec, floor_entity);

            if x == -10 || y == -10 || x == 10 || y == 10 {
                let wall_entity = spawn_wall(vec, &mut commands);
                level.insert(vec, wall_entity);
            }
        }
    }

    spawn_player(GridVector::new(0, 0), 0.0, &mut commands);
    spawn_monster(GridVector::new(5, 5), 0.0, &mut commands);
    spawn_monster(GridVector::new(5, -5), 0.0, &mut commands);
    spawn_monster(GridVector::new(-5, 5), 0.0, &mut commands);
    spawn_monster(GridVector::new(-5, -5), 0.0, &mut commands);

    game_state_next_state.set(GameState::SelectEntity);
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
            Physics { solid: false },
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
            Physics { solid: true },
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
            Physics { solid: true },
            MapEntity,
            Cooldown(cooldown),
            ControlledType::PlayerControlled,
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
            Physics { solid: true },
            MapEntity,
            Cooldown(cooldown),
            ControlledType::AIControlled,
        ))
        .id()
}
