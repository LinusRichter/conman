use ratatui::prelude::Line;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};
use ratatui::widgets::block::Title;
use crate::docker_manager::DockerManager;

#[derive(Clone, Debug)]
pub enum State {
    ContainerSelect(u32),
    OptionSelect(u32),
    StartResultView(String),
    Error
}

#[derive(Clone, Debug)]
pub struct StateManager {
    pub state_stack: Vec<State>,
    pub docker_manager: DockerManager,
    pub select_index: u32
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state_stack: vec![State::ContainerSelect(0)],
            docker_manager: DockerManager::new(),
            select_index: 0
        }
    }
    pub fn next(&mut self) {
        match &self.state_stack.last().unwrap_or(&State::Error) {
            State::ContainerSelect(_) => {
                self.select_index = 0;
                let _ = &self.state_stack.push(State::OptionSelect(0));
            }
            State::OptionSelect(_) => {
                self.select_index = 0;
                let result = self.docker_manager.execute_command(self.collect_states());
                let _ = &self.state_stack.push(State::StartResultView(result));
            }
            State::StartResultView(_) => {
                self.select_index = 0;
                let _ = &self.state_stack.push(State::Error);
            }
            State::Error => {}
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
                Rect::new(x,y,width / 5,height),
                state,
            );
            x += width / 5;
        }
    }
    pub fn draw_state(&self, frame: &mut Frame, area: Rect, state: &State) {
        match state {
            State::ContainerSelect(index) => {
                let container_names: Vec<Line> = self.docker_manager.get_containers()
                    .iter()
                    .enumerate()
                    .map(|(i, &container)| {
                        if i as u32 == *index {
                            return Line::from(container.name)
                                .style(Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::White)
                                );
                        }
                        Line::from(container.name)
                    }).collect();

                frame.render_widget(
                    Paragraph::new(container_names).block(
                        Block::default()
                            .title(Title::from(" Containers ").alignment(Alignment::Center))
                            .border_style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain)
                            .borders(Borders::ALL)
                    ),
                    area
                );
            }
            State::OptionSelect(index) => {
                let container_options: Vec<Line> = self.docker_manager.get_container_options()
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| {
                        if i as u32 == *index { return Line::from(s).style(Style::default().fg(Color::Black).bg(Color::White)); }
                        Line::from(s)
                    }).collect();

                frame.render_widget(
                    Paragraph::new(container_options).block(
                        Block::default()
                            .title(Title::from(" Options ").alignment(Alignment::Center))
                            .border_style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain)
                            .borders(Borders::ALL)
                    ),
                    area
                );
            }
            State::StartResultView(result_string) => {
                frame.render_widget(
                    Paragraph::new(Line::from(result_string.to_string())).block(
                        Block::default()
                            .title(Title::from(" Result ").alignment(Alignment::Center))
                            .border_style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain)
                            .borders(Borders::ALL)
                    ),
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
    pub fn back(&mut self){
        if self.state_stack.len() < 2 {
            return;
        }else {
            self.state_stack.pop();
            self.select_index = match self.get_current_state() {
                State::ContainerSelect(i) => i,
                State::OptionSelect(i) => i,
                State::StartResultView(_) => 0,
                State::Error => 0,
            };
        }
    }
    pub fn update_state_index(&mut self, index: i8) {
        //TODO: simplify
        if index < 0 && self.select_index <= 0 { return; }
        let new_index = if index < 0 { self.select_index - 1 } else { self.select_index + 1 };
        self.select_index = new_index;
        let new_state = match self.get_current_state() {
            State::ContainerSelect(_) => State::ContainerSelect(new_index),
            State::OptionSelect(_) => State::OptionSelect(new_index),
            _ => State::Error
        };
        self.state_stack.pop();
        self.state_stack.push(new_state);
    }
    pub fn get_current_state(&self) -> State {
        self.state_stack.last().unwrap_or(&State::Error).clone()
    }

    pub fn collect_states(&self) -> Vec<u32> {
        let mut collected_states: Vec<u32> = vec![];
        for state in &self.state_stack {
            match state {
                State::ContainerSelect(index) => { collected_states.push(*index); }
                State::OptionSelect(index) => { collected_states.push(*index); }
                _ => {}
            }
        }
        collected_states
    }
}