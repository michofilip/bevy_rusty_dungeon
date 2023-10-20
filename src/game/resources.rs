use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SelectedEntity(pub Option<Entity>);
