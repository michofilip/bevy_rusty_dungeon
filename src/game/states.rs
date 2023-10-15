use bevy::prelude::*;

#[derive(States, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    None,
    SelectEntity,
    SelectBehavior,
    ActionExecution,
}
