use bevy::prelude::*;
use rand::prelude::*;

use crate::game::components::GridPosition;
use crate::game::directions::GridDirection;
use crate::game::model::CharacterType;
use crate::game::services::{actions, player_actions, utils};
use crate::game::vector::GridVector;

pub fn player_behavior(entity: Entity, world: &mut World) -> bool {
    let Some(player_action) = player_actions::get_player_action(world) else {
        return false;
    };

    match player_action {
        player_actions::PlayerAction::Wait => actions::wait(entity, world),
        player_actions::PlayerAction::MoveAttack(direction) => {
            let coordinates = utils::get_shifted_coordinates(entity, direction, world).unwrap();
            actions::attempt_to_attack(entity, coordinates, CharacterType::Monster, world)
                || actions::attempt_to_open_door(entity, coordinates, world)
                || actions::attempt_to_move(
                    entity,
                    coordinates,
                    direction,
                    actions::MoveType::Walk,
                    world,
                )
        }
        player_actions::PlayerAction::Run(direction) => {
            let coordinates = utils::get_shifted_coordinates(entity, direction, world).unwrap();
            actions::attempt_to_move(
                entity,
                coordinates,
                direction,
                actions::MoveType::Run,
                world,
            )
        }
    }
}

pub fn ai_behavior(entity: Entity, world: &mut World) {
    let mut rng = thread_rng();

    let solids = utils::get_static_solids(world);
    let coordinates = world.get::<GridPosition>(entity).unwrap().coordinates;

    let Some((direction, target_coordinates)) = GridDirection::ALL
        .iter()
        .map(|direction| {
            (
                direction,
                coordinates + GridVector::from_direction(&direction),
            )
        })
        .filter(|(_, coordinates)| !solids.contains(coordinates))
        .choose(&mut rng)
    else {
        actions::wait(entity, world);
        return;
    };

    let _ = actions::attempt_to_attack(entity, target_coordinates, CharacterType::Player, world)
        || actions::attempt_to_open_door(entity, target_coordinates, world)
        || actions::attempt_to_move(
            entity,
            target_coordinates,
            direction.to_owned(),
            actions::MoveType::Walk,
            world,
        );
}
