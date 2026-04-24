use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{self as crossterm_terminal, ClearType},
};

use crate::{formatting::status_line, session::Phase};

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn enter(stdout: &mut Stdout) -> Result<Self> {
        crossterm_terminal::enable_raw_mode().context("failed to enable terminal raw mode")?;

        if let Err(error) = execute!(stdout, cursor::Hide) {
            let _ = crossterm_terminal::disable_raw_mode();
            return Err(error).context("failed to hide terminal cursor");
        }

        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm_terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), cursor::Show);
    }
}

pub fn render_status(stdout: &mut Stdout, phase: &Phase, elapsed: Duration) -> Result<()> {
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        crossterm_terminal::Clear(ClearType::CurrentLine),
        Print(status_line(phase, elapsed))
    )
    .context("failed to render status line")?;
    stdout.flush().context("failed to flush status line")
}

pub fn print_line(stdout: &mut Stdout, line: &str) -> Result<()> {
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        crossterm_terminal::Clear(ClearType::CurrentLine),
        Print(line),
        Print("\r\n")
    )
    .context("failed to print line")?;
    stdout.flush().context("failed to flush line")
}
