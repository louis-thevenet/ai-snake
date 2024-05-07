use bevy::prelude::*;

use crate::snake_core::snake::Snake;

use super::neural_network::genetic::GeneticModel;
use super::neural_network::neural_network::{ActionFunction, Layer, NeuralNetwork};
use super::ui::{AppConfig, SimulationState};

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

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_set_up.run_if(in_state(SimulationState::StartUp)),
                one_step_simulation.run_if(in_state(SimulationState::Running)),
            ),
        );
    }
}

fn one_step_simulation(mut app_config: ResMut<Configuration>) {
    println!(">>> doing one step of the simulation");
    let sim = &app_config.simulation;
    for i in 0..sim.population.len() {
        let input = sim.population[i]
            .compute_input(app_config.grid_config.width, app_config.grid_config.height);

        println!("input for individual {}: {:?}", i, input);
        let output = sim.population[i].compute_output(input);

        println!("output for individual {}: {:?}", i, output);
    }
}

fn start_set_up(
    mut commands: Commands,
    app_config: ResMut<AppConfig>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    let config = setup_simulation(
        app_config.grid_size,
        app_config.grid_size,
        app_config.population_size,
    );
    commands.insert_resource(config);

    next_state.set(SimulationState::Paused);
}

fn setup_simulation(width: u64, height: u64, population_count: u64) -> Configuration {
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
    let mut genetic_model = GeneticModel::new(&grid_config, population_count, brains);

    // spawn first snakes
    for i in 0..population_count as usize {
        let snake = Snake::new(width, height, i);
        genetic_model.population[i].add_snake(snake);
        //genetic_model.population[i].universe.spawn_food();
    }

    Configuration {
        simulation: genetic_model,
        grid_config,
    }
}
