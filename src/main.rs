mod statemanager;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, }, ExecutableCommand,
};
use ratatui::{prelude::{CrosstermBackend, Terminal}};
use std::io::{stdout, Result};
use std::time::Duration;
use crossterm::event::{Event, poll};
use crate::statemanager::StateManager;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut state_manager = StateManager::new();
    let mut index: u32 = 0;

    loop {
        terminal.draw(|frame| { state_manager.render(frame); })?;

        if poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Backspace => { state_manager.back(); }
                        KeyCode::Enter => { state_manager.next(index); }
                        KeyCode::Left => { state_manager.back(); }
                        KeyCode::Right => { state_manager.next(index); }
                        KeyCode::Up => {
                            if index >= 1 {
                                index -= 1;
                                state_manager.update_state_index(index);
                            }
                        }
                        KeyCode::Down => {
                            index += 1;
                            state_manager.update_state_index(index);
                        }
                        KeyCode::Esc => { break; }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
