mod state_manager;
mod docker_manager;

use ratatui::widgets::Block;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, }, ExecutableCommand,
};
use ratatui::{prelude::{CrosstermBackend, Terminal}};
use std::io::{Result, stdout};
use std::time::Duration;
use crossterm::event::{Event, poll};
use ratatui::layout::Rect;
use ratatui::prelude::{Alignment, Color, Style};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Borders, BorderType, Paragraph};
use crate::state_manager::StateManager;

#[tokio::main]
async fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let state_manager = StateManager::new().await;
    if state_manager.is_none() {
        terminal.clear()?;
        terminal.draw(|frame|{
            let Rect {
                x,
                y,
                width,
                height
            } = frame.size();

            frame.render_widget(
                Paragraph::new("Error1").block(
                    Block::default()
                        .title(Title::from("Error2").alignment(Alignment::Center))
                        .border_style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain)
                        .borders(Borders::ALL)
                ),
                Rect::new(x,y,width, height - 4)
            );
        });
        std::process::exit(1);
    }

    let mut state_manager = state_manager.unwrap();
    loop {
        terminal.draw(|frame| { state_manager.render(frame); })?;

        if poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Backspace => { state_manager.back(); }
                        KeyCode::Enter => { state_manager.next(); }
                        KeyCode::Left => { state_manager.back(); }
                        KeyCode::Right => { state_manager.next(); }
                        KeyCode::Up => { state_manager.update_state_index(-1); }
                        KeyCode::Down => { state_manager.update_state_index(1); }
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
