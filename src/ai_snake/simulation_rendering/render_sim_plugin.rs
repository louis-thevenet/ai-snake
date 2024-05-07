use bevy::prelude::*;

use super::{
    camera::{camera_controls, spawn_camera},
    grids::display_grid,
    sprites::update_sprites,
};

pub struct RenderSimulationPlugin;

impl Plugin for RenderSimulationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_camera)
            .add_systems(Update, camera_controls)
            .add_systems(Update, display_grid)
            .add_systems(FixedPostUpdate, update_sprites);
    }
}
