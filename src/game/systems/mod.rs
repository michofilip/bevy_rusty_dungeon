use bevy::prelude::*;

use crate::game::systems::game_state_systems::*;
use crate::game::systems::spawn_systems::*;

use super::states::GameState;

mod game_state_systems;
mod spawn_systems;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level)
            .add_systems(
                Update,
                select_entity.run_if(in_state(GameState::SelectEntity)),
            )
            .add_systems(
                Update,
                select_behavior.run_if(in_state(GameState::SelectBehavior)),
            )
            .add_systems(
                Update,
                execute_actions.run_if(in_state(GameState::ActionExecution)),
            );
    }
}
