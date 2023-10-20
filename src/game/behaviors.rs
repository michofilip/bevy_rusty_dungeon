use bevy::{prelude::*, utils::HashSet};
use rand::prelude::*;

use crate::game::components::EntityType;
use crate::game::model::CharacterType;

use super::{
    action::Action,
    components::{GridPosition, Solid},
    direction::GridDirection,
    vector::GridVector,
};

#[derive(Clone)]
pub struct Behavior {
    pub cooldown: f32,
    pub actions: Vec<Action>,
}

impl Behavior {
    pub fn new(cooldown: f32, actions: Vec<Action>) -> Self {
        Self { cooldown, actions }
    }

    pub fn empty(cooldown: f32) -> Self {
        Self::new(cooldown, Vec::new())
    }
}

impl Behavior {
    pub fn select_player_behavior(entity: Entity, world: &mut World) -> Option<Behavior> {
        let key_input = world.resource::<Input<KeyCode>>();

        if key_input.pressed(KeyCode::Numpad5) {
            return Some(Behavior::empty(1.0));
        }

        let Some(direction) = get_direction_from_input(key_input) else {
            return None;
        };

        let solids = get_solids_except(entity, world);

        let Some(position) = world.get::<GridPosition>(entity) else {
            return Some(Behavior::empty(1.0));
        };
        let new_coordinates = position.coordinates + GridVector::from_direction(&direction);

        if solids.contains(&new_coordinates) {
            let mut query = world.query::<(Entity, &GridPosition, &EntityType)>();
            for (target_entity, position, entity_type) in query.iter(world) {
                if position.coordinates != new_coordinates {
                    continue;
                }

                match entity_type {
                    EntityType::Character(character) => {
                        if character.character_type == CharacterType::Monster {
                            return Some(Behavior::new(1.0, vec![Action::Despawn(target_entity)]));
                        }
                    }
                    _ => {}
                }
            }
            // None
            return Some(Behavior::empty(1.0));
        } else {
            Some(Behavior::new(
                1.0,
                vec![
                    Action::SetCoordinates(entity, new_coordinates),
                    Action::SetDirection(entity, direction),
                ],
            ))
        }
    }
}

impl Behavior {
    pub fn select_ai_behavior(entity: Entity, world: &mut World) -> Behavior {
        let mut rng = thread_rng();

        let maybe_direction = GridDirection::ALL.choose(&mut rng);

        if let Some(direction) = maybe_direction {
            let Some(postion) = world.get::<GridPosition>(entity) else {
                return Behavior::empty(1.0);
            };
            let new_coordinates = postion.coordinates + GridVector::from_direction(&direction);
            Behavior::new(
                1.0,
                vec![
                    Action::SetCoordinates(entity, new_coordinates),
                    Action::SetDirection(entity, *direction),
                ],
            )
        } else {
            Behavior::empty(1.0)
        }
    }
}

fn get_direction_from_input(key_input: &Input<KeyCode>) -> Option<GridDirection> {
    if key_input.any_pressed([KeyCode::Up, KeyCode::Numpad8]) {
        Some(GridDirection::North)
    } else if key_input.any_pressed([KeyCode::Right, KeyCode::Numpad6]) {
        Some(GridDirection::East)
    } else if key_input.any_pressed([KeyCode::Down, KeyCode::Numpad2]) {
        Some(GridDirection::South)
    } else if key_input.any_pressed([KeyCode::Left, KeyCode::Numpad4]) {
        Some(GridDirection::West)
    } else if key_input.any_pressed([KeyCode::Numpad9]) {
        Some(GridDirection::NorthEast)
    } else if key_input.any_pressed([KeyCode::Numpad3]) {
        Some(GridDirection::SouthEast)
    } else if key_input.any_pressed([KeyCode::Numpad1]) {
        Some(GridDirection::SouthWest)
    } else if key_input.any_pressed([KeyCode::Numpad7]) {
        Some(GridDirection::NorthWest)
    } else {
        None
    }
}

fn get_solids_except(entity: Entity, world: &mut World) -> HashSet<GridVector> {
    let mut query = world.query_filtered::<(Entity, &GridPosition), With<Solid>>();
    query
        .iter(world)
        .filter(|(e, _)| e != &entity)
        .map(|(_, position)| position.coordinates)
        .collect::<HashSet<GridVector>>()
}
