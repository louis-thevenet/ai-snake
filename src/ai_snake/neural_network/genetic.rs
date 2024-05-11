use std::fmt;

use crate::ai_snake::simulation::GridConfiguration;

use super::{model::Model, NeuralNetwork};

pub struct GeneticModel {
    pub population: Vec<Model>,
}

impl GeneticModel {
    pub fn new(
        grid_config: &GridConfiguration,
        allowed_moves_before_evolution: u64,
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
        GeneticModel { population }
    }

    fn mutate_population(&mut self, mutation_factor: f64) {
        for model in &mut self.population {
            model.brain.mutate(mutation_factor);
        }
    }

    pub fn evolve(&mut self, keep_x_best: f64, mutation_factor: f64) -> (u32, u32, u32) {
        let mut best_score = 0;
        let mut average_score = 0;
        for i in 0..self.population.len() {
            if self.population[i].score > best_score {
                best_score = self.population[i].score;
            }
            average_score += self.population[i].score;
        }
        average_score = (average_score as f32 / self.population.len() as f32) as u32;

        let mut models_to_merge = vec![];
        for i in 0..(self.population.len()) {
            if self.population[i].score >= ((1. - keep_x_best) * best_score as f64) as u32 {
                models_to_merge.push(i);
            }
        }
        let brain_merged = self.merge_brains(&models_to_merge);

        for i in 0..self.population.len() {
            self.population[i].brain = brain_merged.clone();
        }
        let models_merged = models_to_merge.len();
        self.mutate_population(mutation_factor);
        (best_score, average_score, models_merged as u32)
    }

    fn merge_brains(&self, to_keep: &[usize]) -> NeuralNetwork {
        let mut brain = self.population[0].brain.clone();
        for l in 0..brain.layers.len() {
            for i in 0..brain.layers[l].output_dim {
                for j in 0..brain.layers[l].input_dim {
                    let mut sum = 0.;
                    for &k in to_keep.iter() {
                        sum += self.population[k].brain.layers[l].weights[j][i];
                    }
                    brain.layers[l].weights[j][i] = sum / to_keep.len() as f64;
                }
            }
        }
        brain
    }
}

impl fmt::Display for GeneticModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Genetic model with {} models", self.population.len(),)?;

        writeln!(f, "Models:")?;
        let m = self.population.first().ok_or(std::fmt::Error)?;
        write!(f, "{}", m)?;
        Ok(())
    }
}
