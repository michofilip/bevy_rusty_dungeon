use bevy::prelude::*;

use self::{resources::*, systems::*};

pub mod components;
pub mod directions;
mod model;
mod resources;
pub mod services;
mod systems;
pub mod vector;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEntity>()
            .init_resource::<InputCooldown>()
            .init_resource::<AcceptInput>()
            .add_plugins(GameSystemsPlugin);
    }
}
