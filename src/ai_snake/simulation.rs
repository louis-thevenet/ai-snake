use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::camera::{camera_controls, spawn_camera};
use super::genetic::{self, GeneticModel};
use super::neural_network::{Layer, NeuralNetwork};
use crate::rendering::sprites::RenderSpritePlugin;
use crate::snake_core::{snake::Snake, universe::Universe};
#[derive(Resource)]

pub struct Configuration {
    pub simulation: GeneticModel,
    pub grid_config: GridConfiguration,
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
    let grid_config = GridConfiguration {
        width,
        height,
        cell_size: 16.0,
    };

    let population_count = 10;

    let mut brains: Vec<NeuralNetwork> = vec![];
    for _ in 0..population_count {
        let input = 8;
        let output = 8;
        let mut weights = vec![];
        (0..input).for_each(|k| {
            weights.push(vec![]);
            for l in 0..output {
                weights[k].push(rand::random::<f64>());
            }
        });
        let mut brain = NeuralNetwork::new();
        brain.add_layer(Layer::new(
            2,
            4,
            weights,
            super::neural_network::ActionFunction::Relu,
        ));
        brains.push(brain);
    }
    let genetic_model = GeneticModel::new(&grid_config, population_count, brains);

    let config = Configuration {
        simulation: genetic_model,
        grid_config,
    };

    commands.insert_resource(config);
}
fn display_grid(config: Res<Configuration>, mut gizmos: Gizmos) {
    let population = &config.simulation.population;
    let line_length = (1.0 + config.simulation.population.len() as f64).sqrt() as usize;

    for index in 0..population.len() {
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
