use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};

use crate::{
    centered_rect,
    popups::{Popup, PopupType},
    Application,
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
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        let block = Block::default()
            .title("Press q to exit")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Red));
        f.render_widget(Clear, rect);
        f.render_widget(block, rect);
    }

    fn handle_key(
        &mut self,
        key: &KeyEvent,
        app: &Application,
    ) -> (Application, Option<Box<dyn Popup>>) {
        let mut app = app.clone();
        match key.code {
            KeyCode::Char('q') => {
                app.mutable_app_state.running = false;
            }
            _ => {}
        }

        (app, None)
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, self.x_percent, self.y_percent)
    }

    fn popup_type(&self) -> PopupType {
        PopupType::Exit
    }
}
