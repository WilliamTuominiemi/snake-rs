use std::io;

use direction::Direction;
mod direction;

mod game;
use game::Game;

mod snake;
use snake::Snake;

mod apple;
use apple::Apple;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.update()?;
    Ok(())
}
