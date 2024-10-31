use std::collections::HashMap;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Widget},
    Frame,
};

use crate::{
    ui::{components::scrollable_view::ScrollView, login_state::Login, states::State},
    ImutableAppState, MutableAppState, ScreenState,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
    pub offset_x: u16,
    pub offset_y: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Home {
    pub secrets: HashMap<String, String>,
    pub position: Position,
    pub area: Rect,
}

impl Home {
    pub fn new(secrets: HashMap<String, String>, position: Position, area: Rect) -> Self {
        Self {
            secrets,
            position: Position {
                offset_x: position.offset_x,
                offset_y: position.offset_y,
            },
            area,
        }
    }

    pub fn copy_with_secrets(&self, position: Position, area: Rect) -> Self {
        Self {
            secrets: self.secrets.clone(),
            position,
            area,
        }
    }

    pub fn buffer_to_render(&self) -> Buffer {
        // TODO: rework this
        let secrets_count = self.secrets.len();
        let rect = Rect::new(0, 0, 250, secrets_count as u16 * 6 as u16);
        let mut buffer = Buffer::empty(rect);
        let mut y = 0;
        for (key, value) in &self.secrets {
            let key = Text::styled(key, Style::default().fg(Color::White));
            let value = Text::styled(value, Style::default().fg(Color::White));
            let paragraph = Paragraph::new(key).block(Block::default());
            // here width is 250 to showcase scrollable view
            paragraph.render(Rect::new(0, y, 250, 6), &mut buffer);
            y += 1;
            let paragraph = Paragraph::new(value).block(Block::default());
            paragraph.render(Rect::new(0, y, 250, 6), &mut buffer);
            y += 1;
        }

        buffer
    }
}

impl State for Home {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        area: Rect,
    ) {
        match _immutable_state.rect {
            Some(_) => {
                let mut buffer = f.buffer_mut();
                let buffer_to_render = self.buffer_to_render();
                ScrollView::render(&mut buffer, &self.position, area, &buffer_to_render);
            }
            None => {}
        }
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        _immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState) {
        // TODO: rework this
        let mut screen_state = ScreenState::Home(self.clone());
        if key.code == KeyCode::Char('q') {
            screen_state = ScreenState::Login(Login::new(&_immutable_state.db_path));
        }
        if key.code == KeyCode::Char('j') {
            if !ScrollView::check_if_height_out_of_bounds(
                &self.position,
                &self.buffer_to_render(),
                self.area,
            ) {
                screen_state = ScreenState::Home(self.copy_with_secrets(
                    Position {
                        offset_x: self.position.offset_x,
                        offset_y: self.position.offset_y + 1,
                    },
                    _immutable_state.rect.unwrap(),
                ));
            }
        }
        if key.code == KeyCode::Char('k') {
            if self.position.offset_y != 0 {
                screen_state = ScreenState::Home(self.copy_with_secrets(
                    Position {
                        offset_x: self.position.offset_x,
                        offset_y: self.position.offset_y - 1,
                    },
                    _immutable_state.rect.unwrap(),
                ));
            }
        }
        if key.code == KeyCode::Char('h') {
            if self.position.offset_x != 0 {
                screen_state = ScreenState::Home(self.copy_with_secrets(
                    Position {
                        offset_x: self.position.offset_x - 1,
                        offset_y: self.position.offset_y,
                    },
                    _immutable_state.rect.unwrap(),
                ));
            }
        }
        if key.code == KeyCode::Char('l') {
            if !ScrollView::check_if_width_out_of_bounds(
                &self.position,
                &self.buffer_to_render(),
                self.area,
            ) {
                screen_state = ScreenState::Home(self.copy_with_secrets(
                    Position {
                        offset_x: self.position.offset_x + 1,
                        offset_y: self.position.offset_y,
                    },
                    _immutable_state.rect.unwrap(),
                ));
            }
        }
        (mutable_state.clone(), screen_state)
    }
}
