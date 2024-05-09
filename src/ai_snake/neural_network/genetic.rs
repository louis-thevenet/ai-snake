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
        mutation_factor: f64,
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
            mutation_factor,
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

    pub fn evolve(&mut self) -> (u32, u32) {
        let keep_percent = 0.02;
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
            if self.population[i].score >= ((1. - keep_percent) * best_score as f32) as u32 {
                models_to_merge.push(i);
            }
        }
        println!("Merging {} NN", models_to_merge.len());
        let brain_merged = self.merge_brains(models_to_merge);

        for i in 0..self.population.len() {
            self.population[i].brain = brain_merged.clone();
        }

        self.mutate_population();
        (best_score, average_score)
    }
    fn merge_brains(&self, to_keep: Vec<usize>) -> NeuralNetwork {
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
