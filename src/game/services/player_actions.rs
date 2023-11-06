use bevy::prelude::*;

use crate::game::directions::GridDirection;

pub enum PlayerAction {
    Wait,
    MoveAttack(GridDirection),
    Run(GridDirection),
}

pub fn get_player_action(world: &mut World) -> Option<PlayerAction> {
    let key_input = world.resource::<Input<KeyCode>>();

    if key_input.pressed(KeyCode::Numpad5) {
        return Some(PlayerAction::Wait);
    }

    if let Some(direction) = get_direction_from_input(key_input) {
        if key_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            return Some(PlayerAction::Run(direction));
        }

        Some(PlayerAction::MoveAttack(direction))
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
