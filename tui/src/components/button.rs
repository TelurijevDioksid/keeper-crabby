use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::{from, COLOR_ORANGE, COLOR_WHITE};

const BUTTON_HEIGHT: u16 = 3;

pub struct ButtonConfig {
    focused: bool,
    title: String,
}

pub struct Button {}

impl ButtonConfig {
    pub fn new(focused: bool, title: String) -> Self {
        Self { focused, title }
    }

    pub fn height() -> u16 {
        BUTTON_HEIGHT
    }
}

impl Button {
    pub fn render(buffer: &mut Buffer, rect: Rect, config: &ButtonConfig) {
        assert!(rect.height >= BUTTON_HEIGHT);

        let rect = Rect::new(rect.x, rect.y, rect.width, BUTTON_HEIGHT);

        let text = config.title.clone();
        let text = Line::from(text)
            .style(
                Style::default()
                    .fg(from(COLOR_ORANGE).unwrap_or(Color::Yellow))
                    .add_modifier(if config.focused {
                        Modifier::ITALIC
                    } else {
                        Modifier::empty()
                    }),
            )
            .centered();

        let paragraph = Paragraph::new(text).block(Block::bordered().border_style(
            Style::default().fg(if config.focused {
                from(COLOR_ORANGE).unwrap_or(Color::Yellow)
            } else {
                from(COLOR_WHITE).unwrap_or(Color::White)
            }),
        ));

        paragraph.render(rect, buffer);
    }
}
