use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};
use std::time::Duration;

use crate::direction::Direction;
use crate::snake::Snake;

pub struct Game {
    quit: bool,
    width: u16,
    height: u16,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            quit: false,
            width: 20,
            height: 10,
            snake: Snake::new(5, 5),
        }
    }

    pub fn update(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode();

        let mut stdout = io::stdout();

        stdout.execute(cursor::Hide)?;

        while !self.quit {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('q') => self.quit = true,
                        KeyCode::Up => self.snake.change_direction(Direction::Up),
                        KeyCode::Right => self.snake.change_direction(Direction::Right),
                        KeyCode::Down => self.snake.change_direction(Direction::Down),
                        KeyCode::Left => self.snake.change_direction(Direction::Left),
                        _ => {}
                    }
                }
            }

            stdout
                .execute(terminal::Clear(terminal::ClearType::All))?
                .execute(cursor::MoveTo(0, 0))?;
            println!("test");

            self.draw(self.draw_walls(&mut stdout, self.width, self.height));
            self.draw(self.draw_player(
                &mut stdout,
                self.snake.position().0,
                self.snake.position().1,
            ));

            self.update_player_position();

            stdout.flush()?;
        }

        terminal::disable_raw_mode()?;
        stdout.execute(cursor::Show)?;

        Ok(())
    }

    fn update_player_position(&mut self) {
        self.snake.update_position();
        self.snake.keep_within_bounds(self.width, self.height);
    }

    fn draw_walls(
        &self,
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

    fn draw_player(
        &self,
        stdout: &mut io::Stdout,
        x: u16,
        y: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent("█".blue()))?;
        stdout
            .queue(cursor::MoveTo(x + 1, y))?
            .queue(style::PrintStyledContent("█".blue()))?;

        Ok(())
    }

    fn draw(&self, result: Result<(), Box<dyn std::error::Error>>) {
        match result {
            Ok(()) => {}
            Err(e) => println!("Error: {}", e),
        }
    }
}
