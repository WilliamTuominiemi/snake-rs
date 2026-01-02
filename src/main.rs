use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};
use std::time::Duration;

struct Game {
    refresh_rate: u8,
    quit: bool,
}

impl Game {
    pub fn new(refresh_rate: u8) -> Self {
        Self {
            refresh_rate,
            quit: false,
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

            let width = 20;
            let height = 10;

            draw(draw_walls(&mut stdout, width, height));

            stdout.flush()?;
        }

        terminal::disable_raw_mode()?;
        Ok(())
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

fn draw(result: Result<(), Box<dyn std::error::Error>>) {
    match result {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn main() -> io::Result<()> {
    let mut game = Game::new(60);
    game.update()?;
    Ok(())
}
