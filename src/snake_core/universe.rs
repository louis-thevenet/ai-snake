use std::fmt::Display;

use bevy::ecs::system::Resource;
use rand::Rng;

use super::snake::Snake;

#[derive(Debug, PartialEq)]

pub struct Food(pub u64, pub u64);

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Resource)]

pub struct Universe {
    pub width: u64,
    pub height: u64,
    pub snakes: Vec<Snake>,
    pub food: Vec<Food>,
}

impl Universe {
    pub fn new(width: u64, height: u64, snakes: Vec<Snake>) -> Universe {
        Universe {
            width,
            height,
            snakes,
            food: Vec::new(),
        }
    }

    pub fn spawn_food(&mut self) -> (u64, u64) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.width);
        let y = rng.gen_range(0..self.height);
        println!("Spawning food at ({}, {})", x, y);

        for snake in self.snakes.iter() {
            if snake.is_in_pos((x, y)) {
                return self.spawn_food();
            }
        }

        self.food.push(Food(x, y));
        (x, y)
    }

    /// Move the snake and check if it ate something
    pub fn move_snake(&mut self, id: usize, dir: Direction) -> bool {
        let new_tail_pos = self.snakes[id].positions[self.snakes[id].positions.len() - 1];
        match self.snakes[id].move_head(dir, self.width, self.height) {
            Ok(_) => {
                let pos = self.snakes[id].positions[0];
                for (i, food) in self.food.iter().enumerate() {
                    if pos == (food.0, food.1) {
                        self.snakes[id].add_tail(new_tail_pos);
                        self.food.remove(i);
                        return true;
                    }
                }
            }
            Err(_) => {
                self.snakes.remove(id);
            }
        }
        false
    }

    pub fn get_snake(&self, id: usize) -> Option<&Snake> {
        self.snakes.get(id)
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Universe: {} by {}", self.width, self.height)?;

        for snake in &self.snakes {
            write!(f, "{snake}")?;
        }
        Ok(())
    }
}
