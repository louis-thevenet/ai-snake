use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::camera::{camera_controls, spawn_camera};
use crate::rendering::sprites::RenderSpritePlugin;
use crate::snake_core::{snake::Snake, universe::Universe};
#[derive(Resource)]
pub struct Configuration {
    pub sim_config: SimulationConfig,
    pub grid_config: GridConfiguration,
}
pub struct SimulationConfig {
    pub universes: Vec<Universe>,
    pub population: u32,
}

pub struct GridConfiguration {
    pub width: u64,
    pub height: u64,
    pub cell_size: f32,
}
pub struct AISnakePlugin;

impl Plugin for AISnakePlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugins(RenderSpritePlugin);
        app.add_systems(Startup, (setup_game, spawn_camera).chain())
            .add_systems(Update, (camera_controls, display_grid));
    }
}

fn setup_game(mut commands: Commands) {
    let width = 32;
    let height = 32;
    let mut config = Configuration {
        sim_config: SimulationConfig {
            universes: vec![Universe::new(width, height, vec![])],
            population: 8,
        },
        grid_config: GridConfiguration {
            width,
            height,
            cell_size: 16.0,
        },
    };

    for i in 0..config.sim_config.population as usize {
        config
            .sim_config
            .universes
            .insert(i, Universe::new(width, height, vec![]));
    }
    commands.insert_resource(config);
}
fn display_grid(config: Res<Configuration>, mut gizmos: Gizmos) {
    let universes = &config.sim_config.universes;
    let line_length = (1.0 + config.sim_config.population as f64).sqrt() as usize;

    for index in 0..universes.len() {
        let x = ((index % line_length) * config.grid_config.width as usize) as f32
            * config.grid_config.cell_size;
        let y = ((index / line_length) * config.grid_config.width as usize) as f32
            * config.grid_config.cell_size;

        for i in 1..config.grid_config.width {
            gizmos.line_2d(
                Vec2::new(
                    x + i as f32 * config.grid_config.cell_size
                        - (config.grid_config.cell_size
                            + config.grid_config.cell_size * config.grid_config.width as f32)
                            / 2.0,
                    y + -(config.grid_config.cell_size
                        + config.grid_config.cell_size * config.grid_config.height as f32)
                        / 2.0,
                ),
                Vec2::new(
                    x + i as f32 * config.grid_config.cell_size
                        - (config.grid_config.cell_size
                            + config.grid_config.cell_size * config.grid_config.width as f32)
                            / 2.0,
                    y + (config.grid_config.cell_size
                        + config.grid_config.cell_size * (config.grid_config.height - 3) as f32)
                        / 2.0,
                ),
                Color::GRAY,
            );
        }

        for i in 1..config.grid_config.height {
            gizmos.line_2d(
                Vec2::new(
                    x + -(config.grid_config.cell_size
                        + config.grid_config.cell_size * config.grid_config.width as f32)
                        / 2.0,
                    y + i as f32 * config.grid_config.cell_size
                        - (config.grid_config.cell_size
                            + config.grid_config.cell_size * config.grid_config.height as f32)
                            / 2.0,
                ),
                Vec2::new(
                    x + (config.grid_config.cell_size
                        + config.grid_config.cell_size * (config.grid_config.width - 3) as f32)
                        / 2.0,
                    y + i as f32 * config.grid_config.cell_size
                        - (config.grid_config.cell_size
                            + config.grid_config.cell_size * config.grid_config.height as f32)
                            / 2.0,
                ),
                Color::GRAY,
            );
        }
    }
}
