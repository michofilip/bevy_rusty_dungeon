use bevy::{prelude::*, utils::HashSet};
use rand::prelude::*;

use crate::game::action::Action;
use crate::game::behaviors::Behavior;
use crate::game::components::{EntityType, GridPosition, Solid};
use crate::game::directions::GridDirection;
use crate::game::model::CharacterType;
use crate::game::vector::GridVector;

enum ActionPrototype {
    Wait,
    MoveAttack(GridDirection),
    Run(GridDirection),
}

pub fn select_player_behavior(entity: Entity, world: &mut World) -> Option<Behavior> {
    let Some(action_prototype) = get_action_prototype(world) else {
        return None;
    };

    match action_prototype {
        ActionPrototype::Wait => Some(wait()),
        ActionPrototype::MoveAttack(direction) => {
            let position = world.get::<GridPosition>(entity).unwrap();
            let target_coordinates = position.coordinates + GridVector::from_direction(&direction);

            if let Some(target_entity) =
                get_character_at(CharacterType::Monster, target_coordinates, world)
            {
                return Some(attack(entity, target_entity, world));
            }

            if !is_solids_at(target_coordinates, world) {
                return Some(move_to(entity, target_coordinates, direction, 1.0));
            }

            Some(wait())
        }
        ActionPrototype::Run(direction) => {
            let position = world.get::<GridPosition>(entity).unwrap();
            let target_coordinates = position.coordinates + GridVector::from_direction(&direction);

            if !is_solids_at(target_coordinates, world) {
                return Some(move_to(entity, target_coordinates, direction, 0.5));
            }

            Some(wait())
        }
    }
}

pub fn select_ai_behavior(entity: Entity, world: &mut World) -> Behavior {
    let mut rng = thread_rng();

    let solids = get_solids(world);
    let coordinates = world.get::<GridPosition>(entity).unwrap().coordinates;

    if let Some((direction, target_coordinates, target_entity)) = GridDirection::ALL
        .iter()
        .map(|direction| {
            let target_coordinates = coordinates + GridVector::from_direction(&direction);
            (
                direction,
                target_coordinates,
                get_character_at(CharacterType::Player, target_coordinates, world),
            )
        })
        .filter(|(_, coordinates, target_entity)| {
            !solids.contains(coordinates) || target_entity.is_some()
        })
        .choose(&mut rng)
    {
        if let Some(target_entity) = target_entity {
            return attack(entity, target_entity, world);
        }

        return move_to(entity, target_coordinates, direction.to_owned(), 1.0);
    }

    wait()
}

fn get_action_prototype(world: &mut World) -> Option<ActionPrototype> {
    let key_input = world.resource::<Input<KeyCode>>();

    if key_input.pressed(KeyCode::Numpad5) {
        return Some(ActionPrototype::Wait);
    }

    if let Some(direction) = get_direction_from_input(key_input) {
        if key_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            return Some(ActionPrototype::Run(direction));
        }

        Some(ActionPrototype::MoveAttack(direction))
    } else {
        None
    }
}

fn get_direction_from_input(key_input: &Input<KeyCode>) -> Option<GridDirection> {
    let any_pressed = |inputs: Vec<KeyCode>| key_input.any_pressed(inputs);

    if any_pressed(vec![KeyCode::Numpad8, KeyCode::Up]) {
        Some(GridDirection::North)
    } else if any_pressed(vec![KeyCode::Numpad6, KeyCode::Right]) {
        Some(GridDirection::East)
    } else if any_pressed(vec![KeyCode::Numpad2, KeyCode::Down]) {
        Some(GridDirection::South)
    } else if any_pressed(vec![KeyCode::Numpad4, KeyCode::Left]) {
        Some(GridDirection::West)
    } else if any_pressed(vec![KeyCode::Numpad9]) {
        Some(GridDirection::NorthEast)
    } else if any_pressed(vec![KeyCode::Numpad3]) {
        Some(GridDirection::SouthEast)
    } else if any_pressed(vec![KeyCode::Numpad1]) {
        Some(GridDirection::SouthWest)
    } else if any_pressed(vec![KeyCode::Numpad7]) {
        Some(GridDirection::NorthWest)
    } else {
        None
    }
}

fn wait() -> Behavior {
    Behavior::new(1.0, Vec::new())
}

fn move_to(
    entity: Entity,
    coordinates: GridVector,
    direction: GridDirection,
    cooldown: f32,
) -> Behavior {
    Behavior::new(
        cooldown,
        vec![
            Action::SetCoordinates(entity, coordinates),
            Action::SetDirection(entity, direction),
        ],
    )
}

fn attack(source_entity: Entity, target_entity: Entity, world: &mut World) -> Behavior {
    Behavior::new(1.0, vec![Action::Despawn(target_entity)])
}

fn get_solids(world: &mut World) -> HashSet<GridVector> {
    let mut query = world.query_filtered::<&GridPosition, With<Solid>>();
    query
        .iter(world)
        .map(|position| position.coordinates)
        .collect::<HashSet<GridVector>>()
}

fn is_solids_at(coordinates: GridVector, world: &mut World) -> bool {
    let mut query = world.query_filtered::<&GridPosition, With<Solid>>();
    query
        .iter(world)
        .filter(|position| position.coordinates == coordinates)
        .next()
        .is_some()
}

fn get_character_at(
    character_type: CharacterType,
    coordinates: GridVector,
    world: &mut World,
) -> Option<Entity> {
    let mut query = world.query::<(Entity, &GridPosition, &EntityType)>();
    query
        .iter(world)
        .filter(|(_, position, entity_type)| match entity_type {
            EntityType::Character(character) => {
                position.coordinates == coordinates && character.character_type == character_type
            }
            _ => false,
        })
        .map(|(entity, _, _)| entity)
        .next()
}
