use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::game::systems::spawn_systems::*;
use crate::game::systems::turn_systems::*;
use crate::states::MainState;

mod spawn_systems;
mod turn_systems;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_level)
            .add_systems(
                Update,
                process_turn
                    .run_if(in_state(MainState::Game))
                    .run_if(on_timer(Duration::from_millis(100))),
            );
    }
}
