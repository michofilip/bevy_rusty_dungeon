use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::game::components::*;
use crate::game::resources::*;
use crate::game::services;

pub fn update_input_cooldown(
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

        if !processed_entities.insert(entity) {
            break;
        }

        let is_player = is_player(entity, world);

        if is_player && player_acted {
            break;
        }

        if is_player {
            player_acted = services::behaviors::player_behavior(entity, world);
            if player_acted {
                reset_input_cooldown(world);
                selected_entity(world).0 = None;
            }
        } else {
            services::behaviors::ai_behavior(entity, world);
            selected_entity(world).0 = None;
        }
    }
}

fn is_player(entity: Entity, world: &mut World) -> bool {
    world.get::<PlayerControlled>(entity).is_some()
}

fn selected_entity(world: &mut World) -> Mut<SelectedEntity> {
    world.resource_mut::<SelectedEntity>()
}

fn reset_input_cooldown(world: &mut World) {
    world.resource_mut::<InputCooldown>().reset();
    world.resource_mut::<AcceptInput>().0 = false;
}

fn select_next_entity(world: &mut World) -> Option<Entity> {
    let mut next_entity = None;
    let mut min_cooldown = 0.0;

    let mut query = world.query::<(Entity, &Cooldown)>();
    for (entity, cooldown) in query.iter(world) {
        if next_entity == None || cooldown.0 < min_cooldown {
            next_entity = Some(entity);
            min_cooldown = cooldown.0;
        }
    }

    if min_cooldown > 0.0 {
        let mut query = world.query::<&mut Cooldown>();
        for mut cooldown in query.iter_mut(world) {
            cooldown.0 -= min_cooldown;
        }
    }

    selected_entity(world).0 = next_entity;

    next_entity
}
