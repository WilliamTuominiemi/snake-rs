use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};
use std::time::Duration;

use direction::Direction;
mod direction;

struct Game {
    quit: bool,
    width: u16,
    height: u16,
    player_x: u16,
    player_y: u16,
    player_dir: Direction,
}

impl Game {
    pub fn new() -> Self {
        Self {
            quit: false,
            width: 20,
            height: 10,
            player_x: 5,
            player_y: 5,
            player_dir: Direction::Right,
        }
    }

    pub fn update(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode();

        let mut stdout = io::stdout();

        while !self.quit {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('q') => self.quit = true,
                        _ => {}
                    }
                }
            }

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?;
            println!("test");

            draw(draw_walls(&mut stdout, self.width, self.height));
            draw(draw_player(&mut stdout, self.player_x, self.player_y));

            self.update_player_position();

            stdout.flush()?;
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn update_player_position(&mut self) {
        match self.player_dir {
            Direction::Up => self.player_y += 1,
            Direction::Right => self.player_x += 1,
            Direction::Down => self.player_y -= 1,
            Direction::Left => self.player_x -= 1,
        }

        if self.player_x == 0 {
            self.player_x = self.width;
        } else if self.player_x >= self.width - 2 {
            self.player_x = 0;
        }

        if self.player_y == 0 {
            self.player_y = self.height;
        } else if self.player_y >= self.height - 1 {
            self.player_y = 0;
        }
    }
}

fn draw_walls(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    for y in 0..height {
        for x in 0..width {
            if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".green()))?;
            }
        }
    }

    Ok(())
}

fn draw_player(stdout: &mut io::Stdout, x: u16, y: u16) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(cursor::Hide)?;

    stdout
        .queue(cursor::MoveTo(x, y))?
        .queue(style::PrintStyledContent("█".blue()))?;
    stdout
        .queue(cursor::MoveTo(x + 1, y))?
        .queue(style::PrintStyledContent("█".blue()))?;

    Ok(())
}

fn draw(result: Result<(), Box<dyn std::error::Error>>) {
    match result {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.update()?;
    Ok(())
}
