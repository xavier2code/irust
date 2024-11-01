use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

pub fn new_rectangle() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40-1) || (x == 0 || x == 150-1) {
                stdout
                    .queue(cursor::MoveTo(x,y))?
                    .queue(style::PrintStyledContent("█".green()))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}