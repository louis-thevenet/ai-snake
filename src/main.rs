use ai_snake::simulation::AISnakePlugin;
use bevy::{app::App, DefaultPlugins};
use snake_game::game::SnakeGamePlugin;
mod ai_snake;
mod snake_core;
mod snake_game;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AISnakePlugin)
        .run();
}
