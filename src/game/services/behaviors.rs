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
        player_actions::PlayerAction::Wait => {
            actions::wait(entity, world);
            true
        }
        player_actions::PlayerAction::MoveAttack(direction) => {
            let position = world.get::<GridPosition>(entity).unwrap();
            let target_coordinates = position.coordinates + GridVector::from_direction(&direction);

            if let Some(target_entity) =
                utils::get_character_at(CharacterType::Monster, target_coordinates, world)
            {
                actions::attack(entity, target_entity, world);
                true
            } else if !utils::is_solids_at(target_coordinates, world) {
                actions::move_to(
                    entity,
                    target_coordinates,
                    direction,
                    actions::MoveType::Walk,
                    world,
                );
                true
            } else {
                false
            }
        }
        player_actions::PlayerAction::Run(direction) => {
            let position = world.get::<GridPosition>(entity).unwrap();
            let target_coordinates = position.coordinates + GridVector::from_direction(&direction);

            if !utils::is_solids_at(target_coordinates, world) {
                actions::move_to(
                    entity,
                    target_coordinates,
                    direction,
                    actions::MoveType::Run,
                    world,
                );
                true
            } else {
                false
            }
        }
    }
}

pub fn ai_behavior(entity: Entity, world: &mut World) {
    let mut rng = thread_rng();

    let solids = utils::get_solids(world);
    let coordinates = world.get::<GridPosition>(entity).unwrap().coordinates;

    if let Some((direction, target_coordinates, target_entity)) = GridDirection::ALL
        .iter()
        .map(|direction| {
            let target_coordinates = coordinates + GridVector::from_direction(&direction);
            (
                direction,
                target_coordinates,
                utils::get_character_at(CharacterType::Player, target_coordinates, world),
            )
        })
        .filter(|(_, coordinates, target_entity)| {
            !solids.contains(coordinates) || target_entity.is_some()
        })
        .choose(&mut rng)
    {
        if let Some(target_entity) = target_entity {
            actions::attack(entity, target_entity, world);
        } else {
            actions::move_to(
                entity,
                target_coordinates,
                direction.to_owned(),
                actions::MoveType::Walk,
                world,
            );
        }
    } else {
        actions::wait(entity, world);
    }
}
