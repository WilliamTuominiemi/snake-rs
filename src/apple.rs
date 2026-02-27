use crate::position::Position;
use rand::{Rng, rngs::ThreadRng};

pub struct Apple {
    position: Position,
    random: ThreadRng,
    game_width: u16,
    game_height: u16,
}

impl Apple {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::rng();

        Apple {
            position: Position {
                x: rng.random_range(2..width - 3),
                y: rng.random_range(2..height - 2),
            },
            random: rng,
            game_width: width,
            game_height: height,
        }
    }

    pub fn position(&self) -> Position {
        return self.position.clone();
    }

    pub fn replace(&mut self) {
        let (x, y) = self.get_random_position();

        self.position.set(x, y);
    }

    fn get_random_position(&mut self) -> (u16, u16) {
        return (
            self.random.random_range(2..self.game_width - 3),
            self.random.random_range(2..self.game_height - 2),
        );
    }
}
