use bevy::prelude::*;

use crate::assets::resources::AssetList;
use crate::assets::systems::*;
use crate::states::MainState;

pub mod resources;
mod systems;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_systems(Startup, load_tileset)
            .add_systems(
                Update,
                check_asset_loading.run_if(in_state(MainState::LoadAssets)),
            );
    }
}
