use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::game::action::Action;
use crate::game::behaviors::Behavior;
use crate::game::components::*;
use crate::game::resources::*;

pub fn update_no_movement_timer(
    mut accept_input: ResMut<AcceptInput>,
    mut input_cooldown: ResMut<InputCooldown>,
    time: Res<Time>,
) {
    if input_cooldown.tick(time.delta()).finished() {
        accept_input.0 = true;
    }
}

pub fn process_turn(world: &mut World) {
    let mut player_acted = false;
    let mut processed_entities = HashSet::new();

    loop {
        let Some(entity) = selected_entity(world)
            .0
            .or_else(|| select_next_entity(world))
        else {
            break;
        };

        selected_entity(world).0 = Some(entity);

        if !processed_entities.insert(entity) {
            break;
        }

        let is_player = is_player(entity, world);

        if is_player && player_acted {
            break;
        }

        let selected_behavior = if is_player {
            Behavior::select_player_behavior(entity, world)
        } else {
            Some(Behavior::select_ai_behavior(entity, world))
        };

        if let Some(behavior) = selected_behavior {
            let mut cooldown = world.get_mut::<Cooldown>(entity).unwrap();
            cooldown.0 += behavior.cooldown;
            execute_actions(behavior.actions, world);

            if is_player {
                player_acted = true;
                set_input_cooldown(world);
            }

            selected_entity(world).0 = None;
        }
    }
}

fn selected_entity(world: &mut World) -> Mut<SelectedEntity> {
    world.resource_mut::<SelectedEntity>()
}

fn set_input_cooldown(world: &mut World) {
    world.resource_mut::<InputCooldown>().reset();
    world.resource_mut::<AcceptInput>().0 = false;
}

fn select_next_entity(world: &mut World) -> Option<Entity> {
    let mut selected_entity = None;
    let mut min_cooldown = 0.0;

    let mut query = world.query::<(Entity, &Cooldown)>();
    for (entity, cooldown) in query.iter(world) {
        if selected_entity == None || cooldown.0 < min_cooldown {
            selected_entity = Some(entity);
            min_cooldown = cooldown.0;
        }
    }

    if min_cooldown > 0.0 {
        let mut query = world.query::<&mut Cooldown>();
        for mut cooldown in query.iter_mut(world) {
            cooldown.0 -= min_cooldown;
        }
    }

    selected_entity
}

fn is_player(entity: Entity, world: &mut World) -> bool {
    world.get::<PlayerControlled>(entity).is_some()
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
