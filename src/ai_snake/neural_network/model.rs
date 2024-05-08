use std::{fmt, path::Display};

use crate::snake_core::{
    snake::Snake,
    universe::{Direction, Food, Universe},
};

use super::neural_network::NeuralNetwork;
pub struct Model {
    pub universe: Universe,
    pub brain: NeuralNetwork,
    pub score: u32,
    pub allowed_moves_number: u32,
    pub moves_left: u32,

    pub id: usize,
}

impl Model {
    pub fn new(width: u64, height: u64, moves_left: u32, id: usize, brain: NeuralNetwork) -> Self {
        let universe = Universe::new_empty(width, height);
        let score = 0;
        Model {
            universe,
            brain,
            score,
            allowed_moves_number: moves_left,
            moves_left,
            id,
        }
    }
    pub fn reset(&mut self) {
        self.add_snake(Snake::new(self.universe.width, self.universe.height, 0));
        self.score = 0;
        self.moves_left = self.allowed_moves_number;
        if self.universe.food.is_empty() {
            self.universe.spawn_food();
        }
    }
    pub fn compute_input(&self, width: u64, height: u64) -> Option<Vec<f64>> {
        let mut input = vec![];
        let vision_range: i64 = 10;
        if let Some(snake) = self.universe.get_snake(0) {
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
            Some(input)
        } else {
            //println!("No snake in model id {}", self.id);
            None
        }
    }

    pub fn compute_output(&self, input: Vec<f64>) -> Vec<f64> {
        self.brain.forward(input)
    }

    pub fn add_snake(&mut self, snake: Snake) {
        self.universe.add_snake(snake);
    }

    pub fn update_position(&mut self, direction: Direction) {
        if self.moves_left == 0 {
            self.universe.kill_snake(0);
        } else {
            let res = self.universe.move_snake(0, direction);
            self.moves_left -= 1;

            if res {
                self.score += 1;
                self.universe.spawn_food();
            }
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Model {}, NN:\n", self.id)?;
        writeln!(f, "{}", self.brain)?;
        Ok(())
    }
}
