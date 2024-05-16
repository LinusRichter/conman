use ratatui::prelude::Line;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::block::Title;

#[derive(Clone, Debug)]
pub enum State {
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
            state_stack: vec![State::ContainerSelect(0)],
        }
    }
    pub fn next(&mut self, index: u32) -> State {
        match &self.state_stack.last().unwrap_or(&State::Error) {
            State::ContainerSelect(_) => {
                let _ = &self.state_stack.push(State::OptionSelect(index));
                self.get_current_state()
            }
            State::OptionSelect(_) => {
                let _ = &self.state_stack.push(State::Error);
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
            self.draw_state(
                frame,
                Rect::new(x,y,width / 6,height),
                state,
            );

            x += width / 6;
        }
    }
    pub fn draw_state(&self, frame: &mut Frame, area: Rect, state: &State) {
        match state {
            State::ContainerSelect(index) => {
                let container_names: Vec<Line> = vec!["Container1", "Container2", "Container3", "Container4"]
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| {
                        if i as u32 == *index { return Line::from(s).style(Style::default().fg(Color::Black).bg(Color::White)); }
                        Line::from(s)
                    }).collect();

                frame.render_widget(
                    Paragraph::new(container_names).block(
                        Block::default()
                            .title(Title::from("Containers").alignment(Alignment::Center))
                            .border_style(Style::default().fg(Color::White))
                            .border_type(BorderType::Rounded)
                            .borders(Borders::ALL)
                    ),
                    area
                );
            }
            State::OptionSelect(_) => {
                frame.render_widget(
                    Block::default()
                        .title(format!("{:?}", state))
                        .border_style(Style::default().fg(Color::White))
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL),
                    area
                );
            }
            State::Error => {
                frame.render_widget(
                    Block::default()
                        .title(format!("{:?}", state))
                        .border_style(Style::default().fg(Color::White))
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL),
                    area
                );
            }
        };
    }
    pub fn back(&mut self) -> State{
        if self.state_stack.len() < 2 { 
            self.get_current_state()
        }else {
            self.state_stack.pop().unwrap_or(State::Error)
        }
    }
    pub fn update_state_index(&mut self, index: u32) {
        let new_state = match self.get_current_state() {
            State::ContainerSelect(_) => State::ContainerSelect(index),
            State::OptionSelect(_) => State::OptionSelect(index),
            State::Error => State::Error
        };
        self.state_stack.pop();
        self.state_stack.push(new_state);
    }
    pub fn get_current_state(&self) -> State {
        self.state_stack.last().unwrap_or(&State::Error).clone()
    }
}