use bevy::prelude::*;

use super::neural_network::genetic::GeneticModel;
use super::neural_network::neural_network::{ActionFunction, Layer, NeuralNetwork};

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

pub fn setup_simulation(width: u64, height: u64, population_count: u64) -> Configuration {
    let grid_config = GridConfiguration {
        width,
        height,
        cell_size: 16.0,
    };

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

    Configuration {
        simulation: genetic_model,
        grid_config,
    }
}
