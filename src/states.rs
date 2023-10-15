use bevy::prelude::*;

#[derive(States, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}
