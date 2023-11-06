use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetList(pub Vec<UntypedHandle>);

#[derive(Resource, Deref, DerefMut)]
pub struct Tileset(pub Handle<TextureAtlas>);
