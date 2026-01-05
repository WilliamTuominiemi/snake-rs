use std::io;

mod direction;

mod game;
use game::Game;

mod snake;

mod apple;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.update()?;
    Ok(())
}
