use bevy::prelude::*;

use crate::constants::*;
use crate::setup::systems::*;
use crate::states::MainState;

mod systems;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_state::<MainState>()
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: TITLE.into(),
                            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                            resizable: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .build(),
            )
            .add_systems(Update, bevy::window::close_on_esc)
            .add_systems(Startup, spawn_camera);
    }
}
