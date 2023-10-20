use bevy::prelude::*;

use crate::game::action::Action;
use crate::game::behaviors::Behavior;
use crate::game::components::*;
use crate::game::resources::*;

pub fn process_turn(world: &mut World) {
    let mut selected_entity = world.resource_mut::<SelectedEntity>().0;
    if selected_entity.is_none() {
        selected_entity = select_min_entity(world);
    }

    let Some(entity) = selected_entity else {
        return;
    };

    let maybe_behavior = select_behavior(entity, world);

    if let Some(behavior) = maybe_behavior {
        world.resource_mut::<SelectedEntity>().0 = None;
        process_behavior(behavior, entity, world);
    } else {
        world.resource_mut::<SelectedEntity>().0 = Some(entity);
    }
}

fn select_min_entity(world: &mut World) -> Option<Entity> {
    let mut min_entity = None;
    let mut min_cooldown = 0.0;

    let mut query = world.query::<(Entity, &Cooldown)>();
    for (entity, cooldown) in query.iter(world) {
        if min_entity == None || cooldown.0 < min_cooldown {
            min_entity = Some(entity);
            min_cooldown = cooldown.0;
        }
    }

    let mut query = world.query::<&mut Cooldown>();
    if min_cooldown > 0.0 {
        for mut cooldown in query.iter_mut(world) {
            cooldown.0 -= min_cooldown;
        }
    }

    min_entity
}

fn select_behavior(entity: Entity, world: &mut World) -> Option<Behavior> {
    let is_player =
        world.get::<ControlledType>(entity).unwrap() == &ControlledType::PlayerControlled;

    if is_player {
        Behavior::select_player_behavior(entity, world)
    } else {
        Some(Behavior::select_ai_behavior(entity, world))
    }
}

fn process_behavior(behavior: Behavior, entity: Entity, world: &mut World) {
    let behavior_cooldown = behavior.cooldown(world);
    let mut cooldown = world.get_mut::<Cooldown>(entity).unwrap();
    cooldown.0 += behavior_cooldown;

    let actions = behavior.actions(entity, world);
    execute_actions(actions, world);
}

fn execute_actions(actions: Vec<Action>, world: &mut World) {
    let mut actions = actions;
    loop {
        let mut result_actions = Vec::new();

        for action in &actions {
            let result = action.execute(world);

            match result {
                Ok(mut actions) => {
                    result_actions.append(&mut actions);
                }
                Err(_) => {}
            }
        }

        if result_actions.is_empty() {
            return;
        } else {
            actions = result_actions;
        }
    }
}

// fn select_player_behavior(world: &mut World) -> Option<Behavior> {
//     let key_input = world.resource::<Input<KeyCode>>();

//     let maybe_direction = if key_input.any_pressed([KeyCode::Up, KeyCode::Numpad8]) {
//         Some(GridDirection::North)
//     } else if key_input.any_pressed([KeyCode::Right, KeyCode::Numpad6]) {
//         Some(GridDirection::East)
//     } else if key_input.any_pressed([KeyCode::Down, KeyCode::Numpad2]) {
//         Some(GridDirection::South)
//     } else if key_input.any_pressed([KeyCode::Left, KeyCode::Numpad4]) {
//         Some(GridDirection::West)
//     } else if key_input.pressed(KeyCode::Numpad9) {
//         Some(GridDirection::NorthEast)
//     } else if key_input.pressed(KeyCode::Numpad3) {
//         Some(GridDirection::SouthEast)
//     } else if key_input.pressed(KeyCode::Numpad1) {
//         Some(GridDirection::SouthWest)
//     } else if key_input.pressed(KeyCode::Numpad7) {
//         Some(GridDirection::NorthWest)
//     } else {
//         None
//     };

//     if let Some(direction) = maybe_direction {
//         Some(Behavior::Move(direction))
//     } else {
//         None
//     }
// }

// fn select_ai_behavior(world: &mut World) -> Behavior {
//     let mut rng = thread_rng();

//     let maybe_direction = GridDirection::ALL.choose(&mut rng);

//     if let Some(direction) = maybe_direction {
//         Behavior::Move(direction.clone())
//     } else {
//         Behavior::Wait
//     }
// }
