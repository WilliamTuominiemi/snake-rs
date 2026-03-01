use crate::{
    apple::Apple,
    direction::{self, Direction},
    position::Position,
};

pub struct Snake {
    position: Position,
    direction: Direction,
    positions: Vec<Position>,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Snake {
            position: Position { x, y },
            direction: Direction::Right,
            positions: vec![Position { x, y }],
        }
    }

    pub fn position(&self) -> Position {
        return self.position.clone();
    }

    pub fn nodes(&self) -> Vec<Position> {
        return self.positions.clone();
    }

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::Up => self.position.update(0, -1),
            Direction::Right => self.position.update(1, 0),
            Direction::Down => self.position.update(0, 1),
            Direction::Left => self.position.update(-1, 0),
        }
        self.positions.insert(0, self.position.clone());
        self.positions.pop();
    }

    pub fn keep_within_bounds(&mut self, width: u16, height: u16) {
        if self.position.x == 1 && self.direction == Direction::Left {
            self.position.x = width - 2;
        } else if self.position.x >= width - 3 && self.direction == Direction::Right {
            self.position.x = 0;
        }

        if self.position.y == 1 && self.direction == Direction::Up {
            self.position.y = height - 1;
        } else if self.position.y == height - 2 && self.direction == Direction::Down {
            self.position.y = 0;
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match self.direction {
            Direction::Up => {
                if direction != Direction::Down {
                    self.direction = direction;
                }
            }
            Direction::Right => {
                if direction != Direction::Left {
                    self.direction = direction;
                }
            }
            Direction::Down => {
                if direction != Direction::Up {
                    self.direction = direction;
                }
            }
            Direction::Left => {
                if direction != Direction::Right {
                    self.direction = direction;
                }
            }
        };
    }

    pub fn check_collision(&mut self, apple: &mut Apple) {
        if self.collides(apple) {
            apple.replace();
            self.positions.push(self.position.clone());
        }
    }

    fn collides(&self, apple: &Apple) -> bool {
        let apple_position = apple.position();
        let player_position = self.position();

        if player_position.x.abs_diff(apple_position.x) < 2 && player_position.y == apple_position.y
        {
            return true;
        }

        return false;
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

        let position = snake.position();

        assert_eq!(position.x, start_x + 1);
        assert_eq!(position.y, start_y);
    }

    #[test]
    fn test_keeping_within_bounds() {
        let start_x = 5;
        let start_y = 5;

        let width = 20;
        let height = 10;

        let mut snake_within_bounds = Snake::new(start_x, start_y);
        snake_within_bounds.keep_within_bounds(width, height);
        let position = snake_within_bounds.position();

        assert_eq!(start_x, position.x);
        assert_eq!(start_y, position.y);

        let start_x_outside = width;
        let start_y_outside = height;

        let mut snake_outside_bounds = Snake::new(start_x_outside, start_y_outside);
        snake_outside_bounds.keep_within_bounds(width, height);
        let position = snake_outside_bounds.position();

        assert_eq!(1, position.x);
        assert_eq!(1, position.y);
    }
}
