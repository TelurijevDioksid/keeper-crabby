use ratatui::{
    crossterm::event::KeyEvent,
    prelude::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Padding, Paragraph},
    Frame,
};

use crate::{
    ui::{centered_rect, popup::Popup, states::ScreenState},
    ImutableAppState, MutableAppState,
};

#[derive(Clone)]
pub struct MessagePopup {
    pub message: String,
    on_close: fn(
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        previous_state: &ScreenState,
    ) -> (MutableAppState, Option<ScreenState>),
}

impl MessagePopup {
    pub fn new(
        message: String,
        on_close: fn(
            immutable_state: &ImutableAppState,
            mutable_state: &MutableAppState,
            previous_state: &ScreenState,
        ) -> (MutableAppState, Option<ScreenState>),
    ) -> Self {
        MessagePopup { message, on_close }
    }
}

impl Popup for MessagePopup {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
        _current_state: &ScreenState,
    ) {
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
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        _key: &KeyEvent,
        previous_state: &ScreenState,
    ) -> (MutableAppState, Option<ScreenState>) {
        let mut mutable_state = mutable_state.clone();
        mutable_state.popups.pop();
        (self.on_close)(immutable_state, &mutable_state, previous_state)
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, 30, 15)
    }
}
