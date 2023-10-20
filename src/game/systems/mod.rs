use bevy::prelude::*;

use crate::game::systems::spawn_systems::*;
use crate::game::systems::turn_systems::*;
use crate::states::MainState;

mod spawn_systems;
mod turn_systems;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_level)
            .add_systems(Update, process_turn.run_if(in_state(MainState::Game)));
    }
}
