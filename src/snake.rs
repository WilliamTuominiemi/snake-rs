use crate::direction::{self, Direction};

pub struct Snake {
    x: u16,
    y: u16,
    direction: Direction,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Snake {
            x,
            y,
            direction: Direction::Right,
        }
    }

    pub fn position(&self) -> (u16, u16) {
        return (self.x, self.y);
    }

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn keep_within_bounds(&mut self, width: u16, height: u16) {
        if self.x == 0 {
            self.x = width;
        } else if self.x >= width - 2 {
            self.x = 0;
        }

        if self.y == 0 {
            self.y = height;
        } else if self.y >= height - 1 {
            self.y = 0;
        }
    }
}
