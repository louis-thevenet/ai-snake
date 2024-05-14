use std::fmt::Display;

use bevy::ecs::system::Resource;

use super::universe::Direction;

#[derive(Debug, Resource)]
pub struct Snake {
    pub id: usize,
    pub direction: Direction,
    pub positions: Vec<(u64, u64)>,
}

pub enum SnakeException {
    DeadSnakeException,
}

impl Snake {
    pub fn new(width: u64, height: u64, id: usize) -> Self {
        Snake {
            positions: vec![(width / 2, height / 2)],
            direction: Direction::Up,
            id,
        }
    }

    pub fn move_head(
        &mut self,
        direction: Direction,
        width: u64,
        height: u64,
    ) -> Result<(), SnakeException> {
        self.direction = direction;
        let (x, y) = match self.direction {
            Direction::Up => (0, height + 1),
            Direction::Down => (0, height - 1),
            Direction::Left => (width - 1, 0),
            Direction::Right => (width + 1, 0),
        };

        if self.positions.contains(&(
            (self.positions[0].0 + x) % width,
            (self.positions[0].1 + y) % height,
        )) {
            return Err(SnakeException::DeadSnakeException);
        }

        for i in (1..self.positions.len()).rev() {
            self.positions[i] = self.positions[i - 1];
        }

        let old = self.positions[0];

        if (old.0 + x) % width == 0 || (old.1 + y) % height == 0 {
            return Err(SnakeException::DeadSnakeException);
        }

        let new = (
            (old.0 + (x + width)) % width,
            (old.1 + (y + height)) % height,
        );

        if self.positions.contains(&new) {
            return Err(SnakeException::DeadSnakeException);
        }
        self.positions[0] = new;
        Ok(())
    }

    pub fn add_tail(&mut self, pos: (u64, u64)) {
        self.positions.push(pos);
    }

    pub fn is_in_pos(&self, pos: (u64, u64)) -> bool {
        self.positions.contains(&pos)
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Snake {} at {:?}", self.id, self.positions)
    }
}
