use bevy::prelude::*;

use crate::assets::resources::Tileset;
use crate::game::components::*;
use crate::game::directions::GridDirection;
use crate::game::vector::GridVector;

pub fn spawn_game_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Name, &GridPosition, &EntityType), Added<MapEntity>>,
    tileset: Res<Tileset>,
) {
    for (entity, name, grid_position, entity_type) in &entity_query {
        let mut insert_graphics = |layer: f32, sprite_index: usize| {
            commands.entity(entity).insert(spawn_sprite_sheet_bundle(
                grid_position.coordinates,
                layer,
                sprite_index,
                &tileset.0,
            ));
        };

        match entity_type {
            EntityType::Static => match name.as_str() {
                "floor" => insert_graphics(0.0, 0),
                "wall" => insert_graphics(10.0, 1),
                _ => {}
            },
            EntityType::Character(_) => match name.as_str() {
                "player" => insert_graphics(20.0, 8),
                "monster" => insert_graphics(20.0, 9),
                _ => {}
            },
            EntityType::Door(door) => match name.as_str() {
                "door" => insert_graphics(10.0, if door.closed { 3 } else { 2 }),
                _ => {}
            },
            EntityType::Switch(_) => {}
        }
    }
}

fn spawn_sprite_sheet_bundle(
    coordinates: GridVector,
    layer: f32,
    sprite_index: usize,
    tileset: &Handle<TextureAtlas>,
) -> SpriteSheetBundle {
    let translation = coordinates.to_vec3(layer);

    SpriteSheetBundle {
        texture_atlas: tileset.clone(),
        sprite: TextureAtlasSprite::new(sprite_index),
        transform: Transform::from_translation(translation),
        ..default()
    }
}

pub fn update_game_entity_graphics(
    mut entity_query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &GridPosition,
            &EntityType,
        ),
        Or<(Changed<GridPosition>, Changed<EntityType>)>,
    >,
) {
    for (mut transform, mut sprite, position, entity_type) in &mut entity_query {
        let new_translation = position.coordinates.to_vec3(transform.translation.z);
        transform.translation = new_translation;

        if let Some(direction) = position.direction {
            match direction {
                GridDirection::NorthEast | GridDirection::East | GridDirection::SouthEast => {
                    sprite.flip_x = false;
                }
                GridDirection::NorthWest | GridDirection::West | GridDirection::SouthWest => {
                    sprite.flip_x = true;
                }
                _ => {}
            }
        }

        match entity_type {
            EntityType::Static => {}
            EntityType::Character(_) => {}
            EntityType::Door(door) => {
                if door.closed {
                    sprite.index = 3;
                } else {
                    sprite.index = 2;
                }
            }
            EntityType::Switch(_) => {}
        }
    }
}

pub fn update_camera_position(
    player_query: Query<&GridPosition, (Changed<GridPosition>, With<PlayerControlled>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(grid_position) = player_query.get_single() {
        let mut transform = camera_query.single_mut();
        transform.translation.x = grid_position.coordinates.vec_x();
        transform.translation.y = grid_position.coordinates.vec_y();
    }
}
