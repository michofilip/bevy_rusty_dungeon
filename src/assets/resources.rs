use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetList(pub Vec<HandleUntyped>);

#[derive(Resource, Deref, DerefMut)]
pub struct Tileset(pub Handle<TextureAtlas>);
