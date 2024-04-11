use std::fmt::Display;

use bevy::ecs::system::Resource;

use super::universe::Direction;

#[derive(Debug, Resource)]
pub struct Snake {
    pub id: usize,
    pub direction: Direction,
    pub positions: Vec<(u64, u64)>,
}

impl Snake {
    pub fn new(width: u64, height: u64, id: usize) -> Self {
        Snake {
            positions: vec![(width / 2, height / 2)],
            direction: Direction::Up,
            id,
        }
    }

    pub fn move_head(&mut self, direction: Direction, width: u64, height: u64) {
        self.direction = direction;
        let (x, y) = match self.direction {
            Direction::Up => (0, height + 1),
            Direction::Down => (0, height - 1),
            Direction::Left => (width - 1, 0),
            Direction::Right => (width + 1, 0),
        };

        for i in (1..self.positions.len()).rev() {
            self.positions[i] = self.positions[i - 1];
        }

        let old = self.positions[0];
        self.positions[0] = (
            (old.0 + (x + width)) % width,
            (old.1 + (y + height)) % height,
        );
    }

    pub fn add_tail(&mut self) {
        self.positions
            .push(self.positions[self.positions.len() - 1]);
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
