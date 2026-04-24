use std::{
    io::{self, Stdout},
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyEventKind};

use crate::{
    audio::AudioLoop,
    clock::current_time_of_day,
    formatting::{fell_asleep_line, slept_line},
    input::{KeyAction, key_action},
    session::Session,
    terminal::{TerminalGuard, print_line, render_status},
};

const SOUND_PATH: &str = "shh.mp3";
const INPUT_POLL_INTERVAL: Duration = Duration::from_millis(200);

pub fn run() -> Result<()> {
    let _audio = AudioLoop::start(Path::new(SOUND_PATH))?;
    let mut stdout = io::stdout();
    let _terminal = TerminalGuard::enter(&mut stdout)?;

    run_session(&mut stdout)
}

fn run_session(stdout: &mut Stdout) -> Result<()> {
    let mut session = Session::settling();
    let mut phase_started_at = Instant::now();

    loop {
        render_status(stdout, session.phase(), phase_started_at.elapsed())?;

        if !event::poll(INPUT_POLL_INTERVAL).context("failed to poll terminal input")? {
            continue;
        }

        let Event::Key(key) = event::read().context("failed to read terminal input")? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key_action(key) {
            Some(KeyAction::MarkSleeping) => {
                let Some(time_to_sleep) = session.mark_sleeping(phase_started_at.elapsed()) else {
                    continue;
                };
                let time_of_day = current_time_of_day();
                let line = fell_asleep_line(time_to_sleep, &time_of_day);

                print_line(stdout, &line)?;
                phase_started_at = Instant::now();
            }
            Some(KeyAction::Quit) => {
                print_summary(stdout, &session, phase_started_at.elapsed())?;
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn print_summary(stdout: &mut Stdout, session: &Session, elapsed: Duration) -> Result<()> {
    let line = match session.finish(elapsed) {
        Some(sleep_duration) => {
            let time_of_day = current_time_of_day();
            slept_line(sleep_duration, &time_of_day)
        }
        None => "Session ended before baby was marked asleep.".to_owned(),
    };

    print_line(stdout, &line)
}
