use crate::snake_core::{
    snake::Snake,
    universe::{Food, Universe},
};

use super::neural_network::NeuralNetwork;
pub struct Model {
    pub universe: Universe,
    pub brain: NeuralNetwork,
    pub score: u32,
    pub id: usize,
}

impl Model {
    pub fn new(width: u64, height: u64, id: usize, brain: NeuralNetwork) -> Self {
        let universe = Universe::new_empty(width, height);
        let score = 0;
        Model {
            universe,
            brain,
            score,
            id,
        }
    }

    pub fn compute_input(&self, width: u64, height: u64) -> Vec<f64> {
        let mut input = vec![];
        let vision_range: i64 = 20;
        if let Some(snake) = self.universe.get_snake(self.id) {
            for u in -1..=1 {
                for v in -1..=1 {
                    if u == 0 && v == 0 {
                        continue;
                    }
                    input.push(0.0);
                    for i in 1..vision_range {
                        let pos = (
                            snake.positions[0].0 + ((i * u + width as i64) as u64) % width,
                            snake.positions[0].1 + ((i * v + height as i64) as u64) % height,
                        );
                        if snake.is_in_pos(pos) {
                            *input.last_mut().unwrap() = i as f64;
                            break;
                        }
                    }

                    input.push(0.0);
                    for i in 1..vision_range {
                        let pos = (
                            snake.positions[0].0 + ((i * u + width as i64) as u64) % width,
                            snake.positions[0].1 + ((i * v + height as i64) as u64) % height,
                        );
                        if self.universe.food.contains(&Food(pos.0, pos.1)) {
                            *input.last_mut().unwrap() = i as f64;
                            break;
                        }
                    }
                }
            }
        } else {
            println!("No snake in model id {}", self.id);
            println!("{}", self.universe);
        }

        input
    }

    pub fn compute_output(&self, input: Vec<f64>) -> Vec<f64> {
        self.brain.forward(input)
    }

    pub fn add_snake(&mut self, snake: Snake) {
        self.universe.add_snake(snake);
    }
}
