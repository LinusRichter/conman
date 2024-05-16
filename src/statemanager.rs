use std::fmt::format;
use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};

#[derive(Clone, Debug)]
pub enum State {
    Initial,
    ContainerSelect(u32),
    OptionSelect(u32),
    Error
}

#[derive(Clone, Debug)]
pub struct StateManager {
    pub state_stack: Vec<State>
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            state_stack: vec![State::Initial],
        }
    }
    pub fn next(&mut self, index: u32) -> State {
        match &self.state_stack.last().unwrap_or(&State::Error) {
            State::Initial => {
                &self.state_stack.push(State::ContainerSelect(index));
                self.get_current_state()
            }
            State::ContainerSelect(_) => {
                &self.state_stack.push(State::OptionSelect(index));
                self.get_current_state()
            }
            State::OptionSelect(_) => {
                &self.state_stack.push(State::Error);
                self.get_current_state()
            }
            State::Error => State::Error
        }
    }
    pub fn render(&self, frame: &mut Frame) {
        let Rect {
            mut x,
            mut y,
            mut width,
            mut height
        } = frame.size();

        for state in self.state_stack.iter() {

            let cur_block = Block::default()
                .title(format!("{:?}", state))
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL);

            frame.render_widget(
                cur_block,
                Rect::new(x,y,width / 4,height)
            );
            x += width / 4;
        }
    }
    pub fn back(&mut self) -> State{
        if self.state_stack.len() < 2 { 
            self.get_current_state()
        }else {
            self.state_stack.pop().unwrap_or(State::Error)
        }
    }
    pub fn get_current_state(&self) -> State {
        self.state_stack.last().unwrap_or(&State::Error).clone()
    }
}