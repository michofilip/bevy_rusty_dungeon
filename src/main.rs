use bevy::prelude::*;

mod assets;
mod constants;
mod game;
mod graphics;
mod setup;
mod states;

fn main() {
    App::new()
        .add_plugins((
            setup::SetupPlugin,
            assets::AssetsPlugin,
            game::GamePlugin,
            graphics::GraphicsPlugin,
        ))
        .run();
}
