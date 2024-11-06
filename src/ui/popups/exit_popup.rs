use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};

use crate::{
    ui::{centered_rect, popups::Popup, states::ScreenState},
    ImutableAppState, MutableAppState,
};

#[derive(Clone)]
pub struct Exit {
    x_percent: u16,
    y_percent: u16,
}

impl Exit {
    pub fn new() -> Self {
        Exit {
            x_percent: 50,
            y_percent: 50,
        }
    }
}

impl Popup for Exit {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
        _current_state: &ScreenState,
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
        _previous_state: &ScreenState,
    ) -> (MutableAppState, Option<ScreenState>) {
        let mut mutable_state = mutable_state.clone();
        match key.code {
            KeyCode::Char('q') => {
                mutable_state.running = false;
            }
            _ => {}
        }
        (mutable_state, None)
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, self.x_percent, self.y_percent)
    }
}
