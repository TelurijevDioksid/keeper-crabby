use krab_backend::generate_password;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::{
    centered_rect,
    popups::{Popup, PopupType},
    Application,
};

#[derive(Clone)]
pub enum InsertDomainPasswordState {
    Domain,
    Password,
    Confirm,
    Quit,
}

#[derive(Clone, PartialEq)]
pub enum InsertDomainPasswordExitState {
    Confirm,
    Quit,
}

#[derive(Clone)]
pub struct InsertDomainPassword {
    pub domain: String,
    pub password: String,
    pub state: InsertDomainPasswordState,
    pub exit_state: Option<InsertDomainPasswordExitState>,
    x_percent: u16,
    y_percent: u16,
}

impl InsertDomainPassword {
    pub fn new() -> Self {
        InsertDomainPassword {
            domain: String::new(),
            password: String::new(),
            state: InsertDomainPasswordState::Domain,
            exit_state: None,
            x_percent: 40,
            y_percent: 20,
        }
    }

    pub fn domain_append(&mut self, c: char) {
        self.domain.push(c);
    }

    pub fn password_append(&mut self, c: char) {
        self.password.push(c);
    }

    pub fn domain_pop(&mut self) {
        self.domain.pop();
    }

    pub fn password_pop(&mut self) {
        self.password.pop();
    }
}

impl Popup for InsertDomainPassword {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Min(0),
                Constraint::Min(0),
            ])
            .split(rect);

        let text = vec![Line::from(vec![Span::raw(self.domain.clone())])];
        let domain_p = Paragraph::new(text).block(Block::bordered().title("Domain").border_style(
            Style::default().fg(match self.state {
                InsertDomainPasswordState::Domain => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let text = vec![Line::from(vec![Span::raw(self.password.clone())])];
        let password_p =
            Paragraph::new(text).block(Block::bordered().title("Password").border_style(
                Style::default().fg(match self.state {
                    InsertDomainPasswordState::Password => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[2]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertDomainPasswordState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let confirm_p = Paragraph::new(Span::raw("Confirm")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertDomainPasswordState::Confirm => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        f.render_widget(Clear, rect);
        f.render_widget(domain_p, layout[0]);
        f.render_widget(password_p, layout[1]);
        f.render_widget(quit_p, inner_layout[0]);
        f.render_widget(confirm_p, inner_layout[1]);
    }

    fn handle_key(
        &mut self,
        key: &KeyEvent,
        app: &Application,
    ) -> (Application, Option<Box<dyn Popup>>) {
        let mut app = app.clone();
        let mut poped = false;

        match self.state {
            InsertDomainPasswordState::Domain => match key.code {
                KeyCode::Char(c) => {
                    self.domain_append(c);
                }
                KeyCode::Backspace => {
                    self.domain_pop();
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Quit;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    self.state = InsertDomainPasswordState::Password;
                }
                _ => {}
            },
            InsertDomainPasswordState::Password => match key.code {
                KeyCode::Char('g') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.password = generate_password();
                    } else {
                        self.password_append('g');
                    }
                }
                KeyCode::Char(c) => {
                    self.password_append(c);
                }
                KeyCode::Backspace => {
                    self.password_pop();
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Domain;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    self.state = InsertDomainPasswordState::Quit;
                }
                _ => {}
            },
            InsertDomainPasswordState::Quit => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertDomainPasswordExitState::Quit);
                    poped = true;
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Password;
                }
                KeyCode::Right | KeyCode::Tab | KeyCode::Left => {
                    self.state = InsertDomainPasswordState::Confirm;
                }
                KeyCode::Down => {
                    self.state = InsertDomainPasswordState::Domain;
                }
                _ => {}
            },
            InsertDomainPasswordState::Confirm => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertDomainPasswordExitState::Confirm);
                    poped = true;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.state = InsertDomainPasswordState::Quit;
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.state = InsertDomainPasswordState::Domain;
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Password;
                }
                _ => {}
            },
        }

        if !poped {
            app.mutable_app_state.popups.pop();
            app.mutable_app_state.popups.push(Box::new(self.clone()));
            return (app, None);
        }

        (app, Some(Box::new(self.clone())))
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, self.x_percent, self.y_percent)
    }

    fn popup_type(&self) -> PopupType {
        PopupType::InsertDomainPassword
    }
}
