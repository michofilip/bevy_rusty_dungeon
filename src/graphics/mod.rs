use bevy::prelude::*;

use crate::graphics::systems::*;

mod systems;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_game_entity, update_game_entity_graphics));
    }
}
