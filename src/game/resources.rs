use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SelectedEntity(pub Option<Entity>);

#[derive(Resource, Default, PartialEq)]
pub struct AcceptInput(pub bool);

#[derive(Resource, Deref, DerefMut)]
pub struct InputCooldown(pub Timer);

impl InputCooldown {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.15, TimerMode::Once))
    }
}

impl Default for InputCooldown {
    fn default() -> Self {
        Self::new()
    }
}
