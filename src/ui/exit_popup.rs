use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};

use crate::{ImutableAppState, MutableAppState};

use super::popup::Popup;

#[derive(Clone)]
pub struct Exit {}

impl Exit {
    pub fn new() -> Self {
        Exit {}
    }
}

impl Popup for Exit {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
    ) {
        let block = Block::default()
            .title("Press q to exit")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red));
        f.render_widget(Clear, rect);
        f.render_widget(block, rect);
    }

    fn handle_key(
        &mut self,
        _immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        key: &KeyEvent,
    ) -> MutableAppState {
        let mut mutable_state = mutable_state.clone();
        match key.code {
            KeyCode::Char('q') => {
                mutable_state.running = false;
            }
            _ => {}
        }
        mutable_state
    }
}
