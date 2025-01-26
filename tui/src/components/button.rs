use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::{from, COLOR_ORANGE, COLOR_WHITE};

const BUTTON_HEIGHT: u16 = 3;

/// Represents the configuration of a button
///
/// # Fields
/// * `focused` - Indicates if the button is focused
/// * `title` - The title of the button
///
/// # Methods
///
/// * `new` - Creates a new `ButtonConfig`
/// * `height` - Returns the height of the button
pub struct ButtonConfig {
    focused: bool,
    title: String,
}

/// Represents a button
///
/// # Methods
/// * `render` - Renders the button
pub struct Button {}

impl ButtonConfig {
    /// Creates a new `ButtonConfig`
    ///
    /// # Arguments
    /// * `focused` - Indicates if the button is focused
    /// * `title` - The title of the button
    ///
    /// # Returns
    /// A new `ButtonConfig`
    pub fn new(focused: bool, title: String) -> Self {
        Self { focused, title }
    }

    /// Returns the height of the button
    ///
    /// # Returns
    /// The height of the button
    pub fn height() -> u16 {
        BUTTON_HEIGHT
    }
}

impl Button {
    /// Renders the button
    ///
    /// # Arguments
    /// * `buffer` - The mutable buffer to render to
    /// * `rect` - The rectangle to render the button in
    /// * `config` - The configuration of the button
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
