use std::fmt::Display;

use bevy::ecs::system::Resource;

use super::snake::Snake;

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
    snakes: Vec<Snake>,
}

impl Universe {
    pub fn new(width: u64, height: u64, snakes: Vec<Snake>) -> Universe {
        Universe {
            width,
            height,
            snakes,
        }
    }

    pub fn move_snake(&mut self, id: usize, dir: Direction) {
        self.snakes[id].move_head(dir, self.width, self.height);
    }

    pub fn get_snake(&self, id: usize) -> &Snake {
        &self.snakes[id]
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
