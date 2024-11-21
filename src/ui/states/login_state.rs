use std::path::PathBuf;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    crypto::{check_user, user::User},
    ui::{
        centered_rect,
        popups::message_popup::MessagePopup,
        states::{
            home_state::{Home, Position},
            startup_state::StartUp,
            ScreenState, State,
        },
    },
    ImutableAppState, MutableAppState,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginState {
    Username,
    MasterPassword,
    Confirm,
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Login {
    pub username: String,
    pub master_password: String,
    pub state: LoginState,
    pub path: PathBuf,
}

impl Login {
    pub fn username_append(&mut self, c: char) {
        self.username.push(c);
    }

    pub fn master_password_append(&mut self, c: char) {
        self.master_password.push(c);
    }

    pub fn username_pop(&mut self) {
        self.username.pop();
    }

    pub fn master_password_pop(&mut self) {
        self.master_password.pop();
    }

    pub fn new(path: &PathBuf) -> Self {
        Login {
            username: String::new(),
            master_password: String::new(),
            state: LoginState::Username,
            path: path.clone(),
        }
    }

    pub fn copy_with_state(&self, state: LoginState) -> Self {
        Login {
            username: self.username.clone(),
            master_password: self.master_password.clone(),
            state,
            path: self.path.clone(),
        }
    }

    // this needs to be reworked
    // this function should return a vector of cipher configs and a master pwd
    pub fn login(&self) -> Result<User, String> {
        let user_exists = check_user(&self.username, self.path.clone());
        if !user_exists {
            return Err("Cannot login".to_string());
        }

        let user = User::from(&self.path, &self.username, &self.master_password);

        match user {
            Ok(u) => Ok(u),
            Err(_) => Err("Cannot login".to_string()),
        }
    }
}

impl State for Login {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
    ) {
        let rect = centered_rect(rect, 50, 40);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(rect);

        let text = vec![Line::from(vec![Span::raw(self.username.clone())])];
        let username_p =
            Paragraph::new(text).block(Block::bordered().title("Username").border_style(
                Style::default().fg(match self.state {
                    LoginState::Username => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let text = vec![Line::from(vec![Span::raw(self.master_password.clone())])];
        let master_password_p =
            Paragraph::new(text).block(Block::bordered().title("Master Password").border_style(
                Style::default().fg(match self.state {
                    LoginState::MasterPassword => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[2]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                LoginState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let confirm_p = Paragraph::new(Span::raw("Confirm")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                LoginState::Confirm => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        f.render_widget(username_p, layout[0]);
        f.render_widget(master_password_p, layout[1]);
        f.render_widget(quit_p, inner_layout[0]);
        f.render_widget(confirm_p, inner_layout[1]);
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState) {
        let mut mutable_state = mutable_state.clone();
        let mut screen_state = ScreenState::Login(self.clone());

        match self.state {
            LoginState::Username => match key.code {
                KeyCode::Char(c) => {
                    let mut ss = self.copy_with_state(LoginState::Username);
                    ss.username_append(c);
                    screen_state = ScreenState::Login(ss);
                }
                KeyCode::Backspace => {
                    let mut ss = self.copy_with_state(LoginState::Username);
                    ss.username_pop();
                    screen_state = ScreenState::Login(ss);
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    screen_state =
                        ScreenState::Login(self.copy_with_state(LoginState::MasterPassword));
                }
                KeyCode::Up => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Confirm));
                }
                _ => {}
            },
            LoginState::MasterPassword => match key.code {
                KeyCode::Char(c) => {
                    let mut ss = self.copy_with_state(LoginState::MasterPassword);
                    ss.master_password_append(c);
                    screen_state = ScreenState::Login(ss);
                }
                KeyCode::Backspace => {
                    let mut ss = self.copy_with_state(LoginState::MasterPassword);
                    ss.master_password_pop();
                    screen_state = ScreenState::Login(ss);
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Quit));
                }
                KeyCode::Up => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Username));
                }
                _ => {}
            },
            LoginState::Quit => match key.code {
                KeyCode::Enter => {
                    screen_state = ScreenState::StartUp(StartUp::new());
                }
                KeyCode::Right | KeyCode::Left | KeyCode::Tab => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Confirm));
                }
                KeyCode::Up => {
                    screen_state =
                        ScreenState::Login(self.copy_with_state(LoginState::MasterPassword));
                }
                KeyCode::Down => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Username));
                }
                _ => {}
            },
            LoginState::Confirm => match key.code {
                KeyCode::Enter => {
                    let data = self.login();
                    match data {
                        Ok(d) => {
                            screen_state = ScreenState::Home(Home::new(
                                d,
                                Position::default(),
                                immutable_state.rect.unwrap(),
                            ));
                        }
                        Err(e) => {
                            mutable_state.popups.push(Box::new(MessagePopup::new(
                                e,
                                |_, mutable_state: &MutableAppState, _| {
                                    let mut mutable_state = mutable_state.clone();
                                    mutable_state.popups.pop();
                                    (mutable_state, None)
                                },
                            )));
                        }
                    }
                }
                KeyCode::Right | KeyCode::Left => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Quit));
                }
                KeyCode::Up => {
                    screen_state =
                        ScreenState::Login(self.copy_with_state(LoginState::MasterPassword));
                }
                KeyCode::Down | KeyCode::Tab => {
                    screen_state = ScreenState::Login(self.copy_with_state(LoginState::Username));
                }
                _ => {}
            },
        }

        (mutable_state, screen_state)
    }
}
