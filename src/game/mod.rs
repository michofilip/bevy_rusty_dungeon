use bevy::prelude::*;

use self::{resources::*, systems::*};

mod action;
mod behaviors;
pub mod components;
pub mod direction;
mod model;
mod resources;
mod systems;
pub mod vector;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEntity>()
            .add_plugins(GameSystemsPlugin);
    }
}
