#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn update(&mut self, dx: i16, dy: i16) {
        let temp_x = self.x as i16 + dx;
        let temp_y = self.y as i16 + dy;
        self.set(temp_x as u16, temp_y as u16);
    }

    pub fn set(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
