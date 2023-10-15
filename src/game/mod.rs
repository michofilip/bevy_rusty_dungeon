use crate::game::states::GameState;
use bevy::prelude::*;

use self::{resources::*, systems::*};

mod action;
mod behaviors;
pub mod components;
pub mod direction;
mod model;
mod resources;
mod states;
mod systems;
pub mod vector;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .init_resource::<Level>()
            .init_resource::<SelectedEntity>()
            .init_resource::<ActionsToExecute>()
            .add_plugins(GameSystemsPlugin);
    }
}
