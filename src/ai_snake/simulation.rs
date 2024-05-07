use bevy::prelude::*;

use super::neural_network::genetic::GeneticModel;
use super::neural_network::neural_network::{ActionFunction, Layer, NeuralNetwork};
use super::simulation_rendering::render_sim_plugin::RenderSimulationPlugin;

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
        app.add_plugins(RenderSimulationPlugin)
            .add_systems(Startup, setup_simulation);
    }
}

fn setup_simulation(mut commands: Commands) {
    let width = 32;
    let height = 32;
    let grid_config = GridConfiguration {
        width,
        height,
        cell_size: 16.0,
    };

    let population_count = 1;

    let mut brains: Vec<NeuralNetwork> = vec![];
    for _ in 0..population_count {
        let input = 8;
        let output = 8;
        let mut weights = vec![];
        (0..input).for_each(|k| {
            weights.push(vec![]);
            for _ in 0..output {
                weights[k].push(rand::random::<f64>());
            }
        });
        let mut brain = NeuralNetwork::new();
        brain.add_layer(Layer::new(2, 4, weights, ActionFunction::Relu));
        brains.push(brain);
    }
    let genetic_model = GeneticModel::new(&grid_config, population_count, brains);

    let config = Configuration {
        simulation: genetic_model,
        grid_config,
    };

    commands.insert_resource(config);
}
