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
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn keep_within_bounds(&mut self, width: u16, height: u16) {
        if self.x == 0 {
            self.x = width - 3;
        } else if self.x >= width - 2 {
            self.x = 1;
        }

        if self.y == 0 {
            self.y = height - 2;
        } else if self.y >= height - 1 {
            self.y = 1;
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updating_position() {
        let start_x = 5;
        let start_y = 5;

        let mut snake = Snake::new(start_x, start_y); // Start direction is right

        snake.update_position();

        let (x, y) = snake.position();

        assert_eq!(x, start_x + 1);
        assert_eq!(y, start_y);
    }

    #[test]
    fn test_keeping_within_bounds() {
        let start_x = 5;
        let start_y = 5;

        let width = 20;
        let height = 10;

        let mut snake_within_bounds = Snake::new(start_x, start_y);
        snake_within_bounds.keep_within_bounds(width, height);
        let (x, y) = snake_within_bounds.position();

        assert_eq!(start_x, x);
        assert_eq!(start_y, y);

        let start_x_outside = width;
        let start_y_outside = height;

        let mut snake_outside_bounds = Snake::new(start_x_outside, start_y_outside);
        snake_outside_bounds.keep_within_bounds(width, height);
        let (x, y) = snake_outside_bounds.position();

        assert_eq!(1, x);
        assert_eq!(1, y);
    }
}
