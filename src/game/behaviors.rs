use bevy::{prelude::*, utils::HashSet};
use rand::prelude::*;

use super::{
    action::Action,
    components::{GridPosition, Solid},
    direction::GridDirection,
    vector::GridVector,
};

#[derive(Clone, Copy)]
pub enum Behavior {
    Wait,
    Move(GridDirection),
}

impl Behavior {
    pub fn cooldown(&self, world: &mut World) -> f32 {
        match self {
            Behavior::Wait => 1.0,
            Behavior::Move(_) => 1.0,
        }
    }

    pub fn actions(&self, entity: Entity, world: &mut World) -> Vec<Action> {
        match self {
            Behavior::Wait => Vec::new(),
            Behavior::Move(direction) => move_actions(entity, direction, world),
        }
    }
}

fn move_actions(entity: Entity, direction: &GridDirection, world: &mut World) -> Vec<Action> {
    let Some(position) = world.get::<GridPosition>(entity) else {
        return Vec::new();
    };
    let new_coordinates = position.coordinates + GridVector::from_direction(direction);
    vec![
        Action::SetCoordinates(entity, new_coordinates),
        Action::SetDirection(entity, *direction),
    ]
}

impl Behavior {
    pub fn select_player_behavior(entity: Entity, world: &mut World) -> Option<Behavior> {
        let key_input = world.resource::<Input<KeyCode>>();

        if key_input.pressed(KeyCode::Numpad5) {
            return Some(Behavior::Wait);
        }

        let Some(direction) = Self::get_directon_from_input(key_input) else {
            return None;
        };

        let mut query = world.query_filtered::<&GridPosition, With<Solid>>();
        let solids = query
            .iter(world)
            .map(|position| position.coordinates)
            .collect::<HashSet<GridVector>>();

        let Some(postion) = world.get::<GridPosition>(entity) else {
            return Some(Behavior::Wait);
        };
        let new_coordinates = postion.coordinates + GridVector::from_direction(&direction);

        if !solids.contains(&new_coordinates) {
            Some(Behavior::Move(direction))
        } else {
            Some(Behavior::Wait)
        }
    }

    fn get_directon_from_input(key_input: &Input<KeyCode>) -> Option<GridDirection> {
        if key_input.any_pressed([KeyCode::Up, KeyCode::Numpad8]) {
            Some(GridDirection::North)
        } else if key_input.any_pressed([KeyCode::Right, KeyCode::Numpad6]) {
            Some(GridDirection::East)
        } else if key_input.any_pressed([KeyCode::Down, KeyCode::Numpad2]) {
            Some(GridDirection::South)
        } else if key_input.any_pressed([KeyCode::Left, KeyCode::Numpad4]) {
            Some(GridDirection::West)
        } else if key_input.pressed(KeyCode::Numpad9) {
            Some(GridDirection::NorthEast)
        } else if key_input.pressed(KeyCode::Numpad3) {
            Some(GridDirection::SouthEast)
        } else if key_input.pressed(KeyCode::Numpad1) {
            Some(GridDirection::SouthWest)
        } else if key_input.pressed(KeyCode::Numpad7) {
            Some(GridDirection::NorthWest)
        } else {
            None
        }
    }
}

impl Behavior {
    pub fn select_ai_behavior(entity: Entity, world: &mut World) -> Behavior {
        let mut rng = thread_rng();

        let maybe_direction = GridDirection::ALL.choose(&mut rng);

        if let Some(direction) = maybe_direction {
            Behavior::Move(direction.clone())
        } else {
            Behavior::Wait
        }
    }
}
