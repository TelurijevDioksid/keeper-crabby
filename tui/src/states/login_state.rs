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

use krab_backend::{check_user, user::User};

use crate::{
    centered_rect,
    popups::message_popup::MessagePopup,
    states::{
        home_state::{Home, Position},
        startup_state::StartUp,
        ScreenState, State,
    },
    Application,
};

// TODO: change to private (LoginInnerState)
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

    // this needs to be reworked
    // this function should return a vector of cipher configs and a master pwd
    // or does it?
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
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
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

    fn handle_key(&mut self, key: &KeyEvent, app: &Application) -> Application {
        let mut app = app.clone();
        let mut change_state = false;

        match self.state {
            LoginState::Username => match key.code {
                KeyCode::Char(c) => {
                    self.username_append(c);
                }
                KeyCode::Backspace => {
                    self.username_pop();
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = LoginState::MasterPassword;
                }
                KeyCode::Up => {
                    self.state = LoginState::Confirm;
                }
                _ => {}
            },
            LoginState::MasterPassword => match key.code {
                KeyCode::Char(c) => {
                    self.master_password_append(c);
                }
                KeyCode::Backspace => {
                    self.master_password_pop();
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = LoginState::Quit;
                }
                KeyCode::Up => {
                    self.state = LoginState::Username;
                }
                _ => {}
            },
            LoginState::Quit => match key.code {
                KeyCode::Enter => {
                    app.state = ScreenState::StartUp(StartUp::new());
                    change_state = true;
                }
                KeyCode::Right | KeyCode::Left | KeyCode::Tab => {
                    self.state = LoginState::Confirm;
                }
                KeyCode::Up => {
                    self.state = LoginState::MasterPassword;
                }
                KeyCode::Down => {
                    self.state = LoginState::Username;
                }
                _ => {}
            },
            LoginState::Confirm => match key.code {
                KeyCode::Enter => {
                    let data = self.login();
                    match data {
                        Ok(d) => {
                            app.state = ScreenState::Home(Home::new(
                                d,
                                Position::default(),
                                app.immutable_app_state.rect.unwrap(),
                            ));
                            change_state = true;
                        }
                        Err(_) => {
                            app.mutable_app_state
                                .popups
                                .push(Box::new(MessagePopup::new("Cannot login".to_string())));
                        }
                    }
                }
                KeyCode::Right | KeyCode::Left => {
                    self.state = LoginState::Quit;
                }
                KeyCode::Up => {
                    self.state = LoginState::MasterPassword;
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.state = LoginState::Username;
                }
                _ => {}
            },
        }

        if !change_state {
            app.state = ScreenState::Login(self.clone());
        }

        app
    }
}
