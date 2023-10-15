use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::assets::resources::{AssetList, Tileset};
use crate::constants::TILESET_PATH;
use crate::states::MainState;

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    match asset_server.get_group_load_state(asset_list.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            next_state.set(MainState::Game);
        }
        LoadState::Failed => {
            error!("asset loading error");
        }
        _ => {}
    };
}

pub fn load_tileset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<AssetList>,
) {
    let texture = asset_server.load(TILESET_PATH);
    asset_list.0.push(texture.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        texture,
        Vec2::splat(16.0),
        8,
        8,
        Some(Vec2::splat(1.0)),
        Some(Vec2::splat(1.0)),
    );
    let texture_atlas = texture_atlasses.add(atlas);
    commands.insert_resource(Tileset(texture_atlas));
}
