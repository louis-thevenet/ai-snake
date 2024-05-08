use ai_snake::ai_snake_plugin::AISnakePlugin;
use bevy::prelude::PluginGroup;
use bevy::{app::App, render::texture::ImagePlugin, DefaultPlugins};
mod ai_snake;
mod snake_core;
mod snake_game;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AISnakePlugin)
        .run();
}
