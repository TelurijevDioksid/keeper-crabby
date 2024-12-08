use ratatui::{
    crossterm::event::KeyEvent,
    prelude::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Padding, Paragraph},
    Frame,
};

use crate::{
    ui::{
        centered_rect,
        popups::{Popup, PopupType},
    },
    Application,
};

#[derive(Clone)]
pub struct MessagePopup {
    pub message: String,
}

impl MessagePopup {
    pub fn new(message: String) -> Self {
        MessagePopup { message }
    }
}

impl Popup for MessagePopup {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        let message_p = Paragraph::new(self.message.clone())
            .block(
                Block::bordered()
                    .title(" Press any key to continue ")
                    .padding(Padding::new(0, 0, rect.height / 3, 0))
                    .border_style(Style::default().fg(Color::White)),
            )
            .alignment(Alignment::Center);

        f.render_widget(Clear, rect);
        f.render_widget(message_p, rect);
    }

    fn handle_key(
        &mut self,
        _key: &KeyEvent,
        app: &Application,
    ) -> (Application, Option<Box<dyn Popup>>) {
        let mut app = app.clone();
        app.mutable_app_state.popups.pop();

        (app, None)
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, 30, 15)
    }

    fn popup_type(&self) -> PopupType {
        PopupType::Message
    }
}
