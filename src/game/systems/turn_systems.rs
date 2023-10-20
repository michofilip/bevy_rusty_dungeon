use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::game::action::Action;
use crate::game::behaviors::Behavior;
use crate::game::components::*;
use crate::game::resources::*;

pub fn process_turn(world: &mut World) {
    let mut processed_entities = HashSet::new();

    loop {
        let mut selected_entity = get_mut_selected_entity(world).0;
        if selected_entity.is_none() {
            selected_entity = select_min_entity(world);
        }

        let Some(entity) = selected_entity else {
            return;
        };

        if !processed_entities.insert(entity) {
            return;
        }

        let maybe_behavior = if is_player(entity, world) {
            Behavior::select_player_behavior(entity, world)
        } else {
            Some(Behavior::select_ai_behavior(entity, world))
        };

        if let Some(behavior) = maybe_behavior {
            let mut cooldown = world.get_mut::<Cooldown>(entity).unwrap();
            cooldown.0 += behavior.cooldown;
            execute_actions(behavior.actions, world);

            get_mut_selected_entity(world).0 = None;
        } else {
            get_mut_selected_entity(world).0 = Some(entity);
        }

        if is_player(entity, world) {
            return;
        }
    }
}

fn get_mut_selected_entity(world: &mut World) -> Mut<SelectedEntity> {
    world.resource_mut::<SelectedEntity>()
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
