use std::fmt;

use crate::ai_snake::simulation::GridConfiguration;

use super::{model::Model, neural_network::NeuralNetwork};

pub struct GeneticModel {
    pub mutation_factor: f64,
    pub population: Vec<Model>,
}

impl GeneticModel {
    pub fn new(
        grid_config: &GridConfiguration,
        allowed_moves_before_evolution: u32,
        population_count: u64,
        brain: Vec<NeuralNetwork>,
    ) -> Self {
        let mut population: Vec<Model> = Vec::new();
        (0..population_count).for_each(|i| {
            population.push(Model::new(
                grid_config.width,
                grid_config.height,
                allowed_moves_before_evolution,
                i as usize,
                brain[i as usize].clone(),
            ));
        });
        GeneticModel {
            mutation_factor: 0.1,
            population,
        }
    }
    fn mutate_population(&mut self) {
        for model in &mut self.population {
            model.brain.mutate(self.mutation_factor);
        }
    }

    fn copy_best(&mut self, best_model: &Model) {
        self.population
            .iter_mut()
            .for_each(|m| m.brain = best_model.brain.clone());
    }

    fn evolve(&mut self) {
        let mut best_model_index = 0;
        for i in 0..self.population.len() {
            if self.population[i].score > self.population[best_model_index].score {
                best_model_index = i;
            }
        }
        for i in 0..self.population.len() {
            self.population[i].brain = self.population[best_model_index].brain.clone();
        }
        self.mutate_population();
    }
}

impl fmt::Display for GeneticModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Genetic model with {} models, alpha={}",
            self.population.len(),
            self.mutation_factor
        )?;

        writeln!(f, "Models:")?;
        for model in &self.population {
            write!(f, "{}", model)?;
        }
        Ok(())
    }
}
