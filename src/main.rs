use bevy::{app::App, DefaultPlugins};
use snake_game::game::SnakeGamePlugin;
mod rendering;
mod snake_core;
mod snake_game;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakeGamePlugin)
        .run();
}
