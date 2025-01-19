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
pub enum InsertPwdState {
    Domain,
    Pwd,
    Confirm,
    Quit,
}

#[derive(Clone, PartialEq)]
pub enum InsertPwdExitState {
    Confirm,
    Quit,
}

#[derive(Clone)]
pub struct InsertPwd {
    pub domain: String,
    pub pwd: String,
    pub state: InsertPwdState,
    pub exit_state: Option<InsertPwdExitState>,
    x_percent: u16,
    y_percent: u16,
}

impl InsertPwd {
    pub fn new() -> Self {
        InsertPwd {
            domain: String::new(),
            pwd: String::new(),
            state: InsertPwdState::Domain,
            exit_state: None,
            x_percent: 40,
            y_percent: 20,
        }
    }

    pub fn domain_append(&mut self, c: char) {
        self.domain.push(c);
    }

    pub fn pwd_append(&mut self, c: char) {
        self.pwd.push(c);
    }

    pub fn domain_pop(&mut self) {
        self.domain.pop();
    }

    pub fn pwd_pop(&mut self) {
        self.pwd.pop();
    }
}

impl Popup for InsertPwd {
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
                InsertPwdState::Domain => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let text = vec![Line::from(vec![Span::raw(self.pwd.clone())])];
        let pwd_p = Paragraph::new(text).block(Block::bordered().title("Password").border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Pwd => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[2]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let confirm_p = Paragraph::new(Span::raw("Confirm")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Confirm => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        f.render_widget(Clear, rect);
        f.render_widget(domain_p, layout[0]);
        f.render_widget(pwd_p, layout[1]);
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
            InsertPwdState::Domain => match key.code {
                KeyCode::Char(c) => {
                    self.domain_append(c);
                }
                KeyCode::Backspace => {
                    self.domain_pop();
                }
                KeyCode::Up => {
                    self.state = InsertPwdState::Quit;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    self.state = InsertPwdState::Pwd;
                }
                _ => {}
            },
            InsertPwdState::Pwd => match key.code {
                KeyCode::Char('g') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.pwd = generate_password();
                    } else {
                        self.pwd_append('g');
                    }
                }
                KeyCode::Char(c) => {
                    self.pwd_append(c);
                }
                KeyCode::Backspace => {
                    self.pwd_pop();
                }
                KeyCode::Up => {
                    self.state = InsertPwdState::Domain;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    self.state = InsertPwdState::Quit;
                }
                _ => {}
            },
            InsertPwdState::Quit => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertPwdExitState::Quit);
                    poped = true;
                }
                KeyCode::Up => {
                    self.state = InsertPwdState::Pwd;
                }
                KeyCode::Right | KeyCode::Tab | KeyCode::Left => {
                    self.state = InsertPwdState::Confirm;
                }
                KeyCode::Down => {
                    self.state = InsertPwdState::Domain;
                }
                _ => {}
            },
            InsertPwdState::Confirm => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertPwdExitState::Confirm);
                    poped = true;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.state = InsertPwdState::Quit;
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.state = InsertPwdState::Domain;
                }
                KeyCode::Up => {
                    self.state = InsertPwdState::Pwd;
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
        PopupType::InsertPwd
    }
}
