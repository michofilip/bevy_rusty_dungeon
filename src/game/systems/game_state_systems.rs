use bevy::prelude::*;
use rand::prelude::*;

use crate::game::action::Action;
use crate::game::behaviors::Behavior;
use crate::game::components::*;
use crate::game::direction::GridDirection;
use crate::game::resources::*;
use crate::game::states::GameState;

pub fn select_entity(world: &mut World) {
    let mut selected_entity = None;
    let mut min_cooldown = 0.0;

    let mut query = world.query::<(Entity, &Cooldown)>();
    for (entity, cooldown) in query.iter(world) {
        if selected_entity == None || cooldown.0 < min_cooldown {
            selected_entity = Some(entity);
            min_cooldown = cooldown.0;
        }
    }

    let mut query = world.query::<&mut Cooldown>();
    if min_cooldown > 0.0 {
        for mut cooldown in query.iter_mut(world) {
            cooldown.0 -= min_cooldown;
        }
    }

    world.resource_mut::<SelectedEntity>().0 = selected_entity;
    if selected_entity.is_some() {
        set_game_state(GameState::SelectBehavior, world);
    }
}

pub fn select_behavior(world: &mut World) {
    let selected_entity = world.resource_mut::<SelectedEntity>();
    let Some(entity) = selected_entity.0 else {
        set_game_state(GameState::ActionExecution, world);
        return;
    };

    let is_player =
        world.get::<ControlledType>(entity).unwrap() == &ControlledType::PlayerControlled;

    let maybe_behavior = if is_player {
        select_player_behavior(world)
    } else {
        select_ai_behavior(world)
    };

    let Some(behavior) = maybe_behavior else {
        return;
    };

    let behavior_cooldown = behavior.cooldown(world);
    let Some(mut cooldown) = world.get_mut::<Cooldown>(entity) else {
        panic!()
    };
    cooldown.0 += behavior_cooldown;

    add_actions(&mut behavior.actions(entity), world);
    set_game_state(GameState::ActionExecution, world);
}

pub fn execute_actions(world: &mut World) {
    loop {
        let actions = world.resource::<ActionsToExecute>().0.clone();
        clear_actions(world);

        for action in actions {
            let result = action.execute(world);

            match result {
                Ok(mut actions) => {
                    add_actions(&mut actions, world);
                }
                Err(_) => {}
            }
        }

        if world.resource::<ActionsToExecute>().is_empty() {
            break;
        }
    }

    set_game_state(GameState::SelectEntity, world);
}

fn set_game_state(game_state: GameState, world: &mut World) {
    let mut game_state_next_state = world.resource_mut::<NextState<GameState>>();
    game_state_next_state.set(game_state);
}

fn clear_actions(world: &mut World) {
    let mut actions_to_execute = world.resource_mut::<ActionsToExecute>();
    actions_to_execute.clear();
}

fn add_actions(actions: &mut Vec<Action>, world: &mut World) {
    let mut actions_to_execute = world.resource_mut::<ActionsToExecute>();
    actions_to_execute.append(actions);
}

fn select_player_behavior(world: &mut World) -> Option<Behavior> {
    let key_input = world.resource::<Input<KeyCode>>();

    let maybe_direction = if key_input.any_pressed([KeyCode::Up, KeyCode::Numpad8]) {
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
    };

    if let Some(direction) = maybe_direction {
        Some(Behavior::Move(direction))
    } else {
        None
    }
}

fn select_ai_behavior(world: &mut World) -> Option<Behavior> {
    let mut rng = thread_rng();

    let maybe_direction = GridDirection::ALL.choose(&mut rng);

    if let Some(direction) = maybe_direction {
        Some(Behavior::Move(direction.clone()))
    } else {
        Some(Behavior::Wait)
    }
}
