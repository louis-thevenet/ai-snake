use std::cmp;

use bevy::prelude::*;

use crate::snake_core::snake::Snake;

use super::neural_network::genetic::GeneticModel;
use super::neural_network::neural_network::{ActivationFunction, Layer, NeuralNetwork};
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
            FixedUpdate,
            (
                start_set_up.run_if(in_state(SimulationState::StartUp)),
                one_step_simulation.run_if(in_state(SimulationState::Running)),
                evolve.run_if(in_state(SimulationState::Evolving)),
            ),
        );
    }
}

fn one_step_simulation(
    mut sim_config: ResMut<Configuration>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut app_config: ResMut<AppConfig>,
) {
    let width = sim_config.grid_config.width;
    let height = sim_config.grid_config.height;
    let direction = [
        crate::snake_core::universe::Direction::Up,
        crate::snake_core::universe::Direction::Down,
        crate::snake_core::universe::Direction::Left,
        crate::snake_core::universe::Direction::Right,
    ];
    let sim = &mut sim_config.simulation;

    let mut finished = true;
    for i in 0..sim.population.len() {
        if (sim.population[i].moves_left > 0) {
            finished = false;
        }

        // get input for each snake
        if let Some(input) = sim.population[i].compute_input(width, height) {
            // compute output for each snake
            // if i == 0 {
            //     println!("Input For #{}, Score={}", i, sim.population[i].score);
            //     println!("Bodies:");

            //     println!("{:.2} {:.2} {:.2}", input[4], input[8], input[14]);
            //     println!("{:.2} X {:.2}", input[2], input[12]);
            //     println!("{:.2} {:.2} {:.2}", input[0], input[6], input[10]);

            //     println!("\nFood:");

            //     println!("{:.2} {:.2} {:.2}", input[5], input[9], input[15]);
            //     println!("{:.2} X {:.2}", input[3], input[13]);
            //     println!("{:.2} {:.2} {:.2}", input[1], input[7], input[11]);
            //     println!();
            // }
            let output = sim.population[i].compute_output(input);

            // if i == 0 {
            //     println!("Output:");
            //     println!(
            //         "{:.2} {:.2} {:.2} {:.2}",
            //         output[0], output[1], output[2], output[3]
            //     );
            //     println!();
            // }

            // update snake position based on output
            let index_max = output
                .iter()
                .enumerate()
                .max_by(|(_, &a), (_, &b)| a.total_cmp(&b))
                .unwrap()
                .0;
            sim.population[i].update_position(direction[index_max].clone());
        };
    }
    app_config.current_moves += 1;

    if finished {
        evolve(sim_config, next_state, app_config)
    }
}

fn evolve(
    mut sim_config: ResMut<Configuration>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut app_config: ResMut<AppConfig>,
) {
    let sim = &mut sim_config.simulation;

    let (best_score, average_score, models_merged) = sim.evolve();
    println!(
        "[{}] Best: {}, Average: {}, Merged: {}",
        app_config.generation_number, best_score, average_score, models_merged
    );

    for i in 0..sim.population.len() {
        sim.population[i].reset(app_config.allowed_moves);
    }

    app_config.generation_number += 1;
    app_config.current_moves = 0;
    app_config.best_score = best_score as u64;
    app_config.average_score = average_score as u64;
    next_state.set(SimulationState::Running);
}

fn start_set_up(
    mut commands: Commands,
    app_config: ResMut<AppConfig>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    let config = setup_simulation(
        app_config.grid_size,
        app_config.grid_size,
        app_config.allowed_moves,
        app_config.population_size,
    );
    commands.insert_resource(config);

    next_state.set(SimulationState::Paused);
}

fn setup_simulation(
    width: u64,
    height: u64,
    allowed_moves: u32,
    population_count: u64,
) -> Configuration {
    let grid_config = GridConfiguration {
        width,
        height,
        cell_size: 1.0,
    };

    let mut brains: Vec<NeuralNetwork> = vec![];
    for _ in 0..population_count {
        let inner_input_size = 16;
        let inner_output_size = 16;
        let mut weights1 = vec![vec![0.0; inner_input_size]; inner_output_size];

        (0..weights1.len()).for_each(|i| {
            for j in 0..weights1[i].len() {
                weights1[i][j] = rand::random::<f64>() * 2. - 1.;
            }
        });

        let mut biases1 = vec![0.; inner_output_size];
        for b in biases1.iter_mut() {
            *b = rand::random::<f64>() * 2. - 1.;
        }

        let inner_input_size = 16;
        let inner_output_size = 16;
        let mut weights2 = vec![vec![0.0; inner_input_size]; inner_output_size];

        (0..weights2.len()).for_each(|i| {
            for j in 0..weights2[i].len() {
                weights2[i][j] = rand::random::<f64>() * 2. - 1.;
            }
        });

        let mut biases2 = vec![0.; inner_output_size];
        for b in biases2.iter_mut() {
            *b = rand::random::<f64>() * 2. - 1.;
        }

        let final_size = 4;
        let mut weights3 = vec![vec![0.0; final_size]; inner_output_size];

        (0..weights3.len()).for_each(|i| {
            for j in 0..weights3[i].len() {
                weights3[i][j] = rand::random::<f64>() * 2. - 1.;
            }
        });

        let mut biases3 = vec![0.; final_size];
        for b in biases3.iter_mut() {
            *b = rand::random::<f64>() * 2. - 1.;
        }

        let mut brain = NeuralNetwork::new();
        brain
            .add_layer(Layer::new(
                inner_input_size,
                inner_output_size,
                weights1,
                biases1,
                ActivationFunction::Identity,
            ))
            .add_layer(Layer::new(
                inner_input_size,
                inner_output_size,
                weights2,
                biases2,
                ActivationFunction::Identity,
            ))
            .add_layer(Layer::new(
                inner_output_size,
                final_size,
                weights3,
                biases3,
                ActivationFunction::Softmax,
            ));
        brains.push(brain);
    }
    let mutation_factor = 0.001;
    let mut genetic_model = GeneticModel::new(
        &grid_config,
        allowed_moves,
        population_count,
        mutation_factor,
        brains,
    );

    // spawn first snakes
    for i in 0..population_count as usize {
        genetic_model.population[i].reset(allowed_moves);
    }

    println!("{genetic_model}");

    Configuration {
        simulation: genetic_model,
        grid_config,
    }
}
