use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use crate::{from, COLOR_ORANGE, COLOR_WHITE};

const INPUT_HEIGHT: u16 = 3;
const MAX_INPUT_WIDTH: u16 = 32;
const PADDING: u16 = 2;

pub struct InputConfig {
    focused: bool,
    value: String,
    hidden: bool,
    title: String,
    cursor_position: Option<u16>,
}

pub struct Input {}

impl InputConfig {
    pub fn new(
        focused: bool,
        value: String,
        hidden: bool,
        title: String,
        cursor_position: Option<u16>,
    ) -> Self {
        Self {
            focused,
            value,
            hidden,
            title,
            cursor_position,
        }
    }

    pub fn height() -> u16 {
        INPUT_HEIGHT
    }

    pub fn width() -> u16 {
        MAX_INPUT_WIDTH + 2 * PADDING
    }
}

impl Input {
    pub fn render(buffer: &mut Buffer, rect: Rect, config: &InputConfig) {
        assert!(rect.height >= INPUT_HEIGHT);
        assert_eq!(config.focused ^ config.cursor_position.is_some(), false);

        let rect = Rect::new(rect.x, rect.y, rect.width, INPUT_HEIGHT);

        let text = if config.hidden {
            let mut hidden_text = String::new();
            for _ in 0..config.value.len() {
                hidden_text.push('*');
            }
            hidden_text
        } else {
            config.value.clone()
        };

        let text = if config.focused {
            let mut first_part = text.clone();
            let mut second_part =
                first_part.split_off(config.cursor_position.unwrap_or(0) as usize);

            if second_part.len() > 0 {
                second_part = second_part.split_off(1);
            }

            let first_part_len = first_part.len() as u16;
            let second_part_len = second_part.len() as u16;

            let mut line = vec![
                Span::raw(first_part)
                    .style(Style::default().fg(from(COLOR_WHITE).unwrap_or(Color::White))),
                Span::raw("█")
                    .style(Style::default().fg(from(COLOR_ORANGE).unwrap_or(Color::Yellow))),
                Span::raw(second_part)
                    .style(Style::default().fg(from(COLOR_WHITE).unwrap_or(Color::White))),
            ];

            if first_part_len + second_part_len < MAX_INPUT_WIDTH {
                line.push(Span::raw(" ".repeat(
                    (MAX_INPUT_WIDTH - first_part_len - second_part_len) as usize,
                )));
            }

            let text = Line::from(line).centered();

            text
        } else {
            let text_len = text.len() as u16;

            Line::from(vec![
                Span::raw(text)
                    .style(Style::default().fg(from(COLOR_WHITE).unwrap_or(Color::White))),
                Span::raw(" ".repeat((MAX_INPUT_WIDTH - text_len) as usize)),
            ])
            .centered()
        };

        let paragraph = Paragraph::new(text).block(
            Block::bordered()
                .border_style(Style::default().fg(if config.focused {
                    from(COLOR_ORANGE).unwrap_or(Color::Yellow)
                } else {
                    from(COLOR_WHITE).unwrap_or(Color::White)
                }))
                .title(" ".to_string() + &config.title + " ")
                .title_style(Style::default().fg(if config.focused {
                    from(COLOR_ORANGE).unwrap_or(Color::Yellow)
                } else {
                    from(COLOR_WHITE).unwrap_or(Color::White)
                })),
        );

        paragraph.render(rect, buffer);
    }

    pub fn handle_key(
        key: &KeyEvent,
        config: &InputConfig,
        previous_value: String,
    ) -> (String, u16) {
        let mut value = previous_value.clone();
        let mut cursor_position = config.cursor_position.unwrap_or(value.len() as u16);

        match key.code {
            KeyCode::Char(c) => {
                if value.len() as u16 == MAX_INPUT_WIDTH {
                    return (value, cursor_position);
                }
                value.insert(cursor_position as usize, c);
                cursor_position += 1;
            }
            KeyCode::Backspace => {
                if cursor_position > 0 {
                    value.remove(cursor_position as usize - 1);
                    cursor_position -= 1;
                }
            }
            KeyCode::Delete => {
                if cursor_position < value.len() as u16 {
                    value.remove(cursor_position as usize);
                }
                if cursor_position > value.len() as u16 {
                    cursor_position = value.len() as u16;
                }
            }
            KeyCode::Left => {
                if cursor_position > 0 {
                    cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if cursor_position == MAX_INPUT_WIDTH {
                    return (value, cursor_position);
                }
                if cursor_position < value.len() as u16 {
                    cursor_position += 1;
                }
            }
            _ => {}
        }

        (value, cursor_position)
    }
}
