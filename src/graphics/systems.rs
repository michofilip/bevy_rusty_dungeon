use bevy::prelude::*;

use crate::assets::resources::Tileset;
use crate::game::components::*;
use crate::game::direction::GridDirection;
use crate::game::vector::GridVector;

pub fn spawn_game_entity(
    mut commands: Commands,
    entity_query: Query<(Entity, &Name, &GridPosition), Added<MapEntity>>,
    tileset: Res<Tileset>,
) {
    for (entity, name, grid_position) in &entity_query {
        let mut insert_graphics = |layer: f32, sprite_index: usize| {
            commands.entity(entity).insert(spawn_sprite_sheet_bundle(
                grid_position.coordinates,
                layer,
                sprite_index,
                &tileset.0,
            ));
        };

        match name.as_str() {
            "floor" => insert_graphics(0.0, 0),
            "wall" => insert_graphics(10.0, 1),
            "player" => insert_graphics(20.0, 8),
            "monster" => insert_graphics(20.0, 9),
            _ => {}
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
        (&mut Transform, &mut TextureAtlasSprite, &GridPosition),
        Changed<GridPosition>,
    >,
) {
    for (mut transform, mut sprite, position) in &mut entity_query {
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
    }
}
