use rand::Rng;

pub struct Apple {
    x: u16,
    y: u16,
}

impl Apple {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::rng();
        Apple {
            x: rng.random_range(2..width - 3),
            y: rng.random_range(2..height - 2),
        }
    }

    pub fn position(&self) -> (u16, u16) {
        return (self.x, self.y);
    }
}
