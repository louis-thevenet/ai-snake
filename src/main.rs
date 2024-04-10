use bevy::{app::App, DefaultPlugins};
use snake_game::SnakeGamePlugin;
mod snake;
mod snake_game;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakeGamePlugin)
        .run();
}
