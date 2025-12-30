use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};

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
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let width = 60;
    let height = 20;

    draw(draw_walls(&mut stdout, width, height));

    stdout.flush()?;
    Ok(())
}
