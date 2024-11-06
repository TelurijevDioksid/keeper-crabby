use std::collections::HashMap;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Text,
    widgets::Widget,
    Frame,
};

use crate::{
    ui::{
        components::scrollable_view::ScrollView,
        states::{login_state::Login, State},
    },
    ImutableAppState, MutableAppState, ScreenState,
};

const SELECTED_DOMAIN_PWD_BG_COLOR: Color = Color::Rgb(202, 220, 252);
const SELECTED_DOMAIN_PWD_FG_COLOR: Color = Color::Rgb(0, 36, 107);
const DOMAIN_PWD_LIST_ITEM_HEIGHT: u16 = 4;
const RIGHT_PADDING: u16 = 6;

#[derive(Debug, Clone, PartialEq)]
pub struct Secrets {
    pub secrets: Vec<(String, String)>,
    pub selected_secret: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position {
    pub offset_x: u16,
    pub offset_y: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Home {
    pub secrets: Secrets,
    pub position: Position,
    pub area: Rect,
}

impl Home {
    pub fn new(secrets: HashMap<String, String>, position: Position, area: Rect) -> Self {
        let secrets = Secrets {
            secrets: secrets.into_iter().collect(),
            selected_secret: 0,
        };
        Self {
            secrets,
            position: Position {
                offset_x: position.offset_x,
                offset_y: position.offset_y,
            },
            area,
        }
    }

    fn copy_with_secrets(&self, position: Position, area: Rect) -> Self {
        Self {
            secrets: self.secrets.clone(),
            position,
            area,
        }
    }

    fn up(&self, area: Rect) -> Self {
        if self.secrets.selected_secret <= 1 {
            return self.scroll_to_top(area);
        }
        self.set_selected_secret(
            self.secrets.selected_secret - 1,
            self.secrets.selected_secret,
            area,
        )
    }

    fn scroll_to_top(&self, area: Rect) -> Self {
        Self {
            secrets: Secrets {
                secrets: self.secrets.secrets.clone(),
                selected_secret: 0,
            },
            position: Position {
                offset_x: self.position.offset_x,
                offset_y: 0,
            },
            area,
        }
    }

    fn down(&self, area: Rect) -> Self {
        if self.secrets.selected_secret == self.secrets.secrets.len() - 1 {
            return self.scroll_to_bottom(area);
        }
        self.set_selected_secret(
            self.secrets.selected_secret + 1,
            self.secrets.selected_secret,
            area,
        )
    }

    fn scroll_to_bottom(&self, area: Rect) -> Self {
        let max_offset_y = self.buffer_to_render().area().height
            - ScrollView::inner_buffer_bounding_box(area).1
            + 1;
        Self {
            secrets: Secrets {
                secrets: self.secrets.secrets.clone(),
                selected_secret: self.secrets.secrets.len() - 1,
            },
            position: Position {
                offset_x: self.position.offset_x,
                offset_y: max_offset_y,
            },
            area,
        }
    }

    fn set_selected_secret(
        &self,
        selected_secret: usize,
        previous_selected_secret: usize,
        area: Rect,
    ) -> Self {
        assert!(selected_secret < self.secrets.secrets.len());
        let bounding_box = ScrollView::inner_buffer_bounding_box(area);
        let mut position = self.position.clone();
        if selected_secret > previous_selected_secret {
            if selected_secret as u16 * DOMAIN_PWD_LIST_ITEM_HEIGHT + 1
                >= bounding_box.1 + position.offset_y
            {
                position.offset_y += DOMAIN_PWD_LIST_ITEM_HEIGHT;
            }
        } else {
            if selected_secret as u16 * DOMAIN_PWD_LIST_ITEM_HEIGHT + 1 <= position.offset_y {
                position.offset_y -= DOMAIN_PWD_LIST_ITEM_HEIGHT;
            }
        }
        Self {
            secrets: Secrets {
                secrets: self.secrets.secrets.clone(),
                selected_secret,
            },
            position,
            area,
        }
    }

    fn separator(&self, width: u16) -> Text {
        let mut separator = String::new();
        for _ in 0..width {
            separator.push_str("╍");
        }
        Text::styled(separator, Style::default().fg(Color::White))
    }

    fn current_secret_cursor(&self, height: u16, width: u16, index: u16, style: Style) -> Text {
        let mut cursor = String::new();
        for _ in 0..height {
            if self.secrets.selected_secret == index as usize {
                for _ in 0..width - 1 {
                    cursor.push_str(">");
                }
                cursor.push_str("\n");
            } else {
                for _ in 0..width - 1 {
                    cursor.push_str(" ");
                }
                cursor.push_str("\n");
            }
        }
        Text::styled(cursor, style)
    }

    fn width(&self) -> u16 {
        let mut width = 0;
        for (key, value) in self.secrets.secrets.iter() {
            let domain_pwd = format!("  {}: {}", key, value);
            if domain_pwd.len() as u16 > width {
                width = domain_pwd.len() as u16;
            }
        }
        width = width + RIGHT_PADDING;
        if width > self.area.width / 2 {
            width
        } else {
            self.area.width / 2
        }
    }

    fn render_secrets(&self, buffer: &mut Buffer, cursor_offset: u16) {
        let mut y = 0;
        let mut index = 0;
        for (key, value) in self.secrets.secrets.iter() {
            let style = if self.secrets.selected_secret == index {
                Style::default()
                    .bg(SELECTED_DOMAIN_PWD_BG_COLOR)
                    .fg(SELECTED_DOMAIN_PWD_FG_COLOR)
            } else {
                Style::default()
            };
            let cursor = self.current_secret_cursor(3, cursor_offset, index as u16, style);
            let width = self.width();
            if y == 0 {
                cursor.render(Rect::new(0, y + 1, cursor_offset, 3), buffer);
                let separator = self.separator(buffer.area().width);
                separator.render(Rect::new(cursor_offset, y, width, 1), buffer);
                y += 1;
            } else {
                cursor.render(Rect::new(0, y, cursor_offset, 3), buffer);
            }
            let domain_pwd = format!("\n  {}: {}\n", key, value);
            let domain_pwd_text = Text::styled(domain_pwd, style);
            domain_pwd_text.render(Rect::new(cursor_offset, y, width, 3), buffer);
            y += 3;
            let separator = self.separator(buffer.area().width);
            separator.render(Rect::new(cursor_offset, y, width, 1), buffer);
            y += 1;
            index += 1;
        }
    }

    fn buffer_to_render(&self) -> Buffer {
        let cursor_offset = 4;
        let secrets_count = self.secrets.secrets.len();
        let rect = Rect::new(
            0,
            0,
            self.width() + cursor_offset,
            (secrets_count as u16 * DOMAIN_PWD_LIST_ITEM_HEIGHT) + 1,
        );
        let mut buffer = Buffer::empty(rect);
        self.render_secrets(&mut buffer, cursor_offset);

        buffer
    }
}

impl State for Home {
    fn render(
        &self,
        f: &mut Frame,
        immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        area: Rect,
    ) {
        match immutable_state.rect {
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
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState) {
        // TODO: rework this
        let mut screen_state = ScreenState::Home(self.clone());
        if key.code == KeyCode::Char('q') {
            screen_state = ScreenState::Login(Login::new(&immutable_state.db_path));
        }
        if key.code == KeyCode::Char('j') {
            screen_state = ScreenState::Home(self.down(immutable_state.rect.unwrap()));
        }
        if key.code == KeyCode::Char('k') {
            screen_state = ScreenState::Home(self.up(immutable_state.rect.unwrap()));
        }
        if key.code == KeyCode::Char('h') {
            if self.position.offset_x != 0 {
                screen_state = ScreenState::Home(self.copy_with_secrets(
                    Position {
                        offset_x: self.position.offset_x - 1,
                        offset_y: self.position.offset_y,
                    },
                    immutable_state.rect.unwrap(),
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
                    immutable_state.rect.unwrap(),
                ));
            }
        }
        (mutable_state.clone(), screen_state)
    }
}
