use std::{collections::HashMap, path::PathBuf};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    Frame,
};

use krab_backend::{
    check_user,
    user::{ReadOnlyRecords, User},
};

use crate::{
    centered_absolute_rect,
    components::{
        button::{Button, ButtonConfig},
        input::{Input, InputConfig},
    },
    popups::message::MessagePopup,
    states::{
        home::{Home, Position},
        startup::StartUp,
        ScreenState, State,
    },
    Application,
};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum LoginInput {
    Username,
    MasterPassword,
}

#[derive(Debug, Clone, PartialEq)]
enum LoginButton {
    Confirm,
    Quit,
}

// TODO: change to private (LoginInnerState)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginState {
    Username,
    MasterPassword,
    Confirm,
    Quit,
}

#[derive(Debug, Clone)]
pub struct Login {
    username: String,
    master_password: String,
    state: LoginState,
    path: PathBuf,
    cursors: HashMap<LoginInput, u16>,
}

impl Login {
    pub fn new(path: &PathBuf) -> Self {
        let mut cursors = HashMap::new();
        cursors.insert(LoginInput::Username, 0);
        cursors.insert(LoginInput::MasterPassword, 0);
        Login {
            username: String::new(),
            master_password: String::new(),
            state: LoginState::Username,
            path: path.clone(),
            cursors,
        }
    }

    fn login(&self) -> Result<(User, ReadOnlyRecords), String> {
        let user_exists = check_user(&self.username, self.path.clone());
        if !user_exists {
            return Err("Cannot login".to_string());
        }

        let user_creation_result = User::from(&self.path, &self.username, &self.master_password);

        match user_creation_result {
            Ok(u) => Ok(u),
            Err(_) => Err("Cannot login".to_string()),
        }
    }

    fn generate_input_config(&self, input: LoginInput) -> InputConfig {
        match input {
            LoginInput::Username => InputConfig::new(
                self.state == LoginState::Username,
                self.username.clone(),
                false,
                "Username".to_string(),
                if self.state == LoginState::Username {
                    Some(self.cursors.get(&LoginInput::Username).unwrap().clone())
                } else {
                    None
                },
            ),
            LoginInput::MasterPassword => InputConfig::new(
                self.state == LoginState::MasterPassword,
                self.master_password.clone(),
                true,
                "Master Password".to_string(),
                if self.state == LoginState::MasterPassword {
                    Some(
                        self.cursors
                            .get(&LoginInput::MasterPassword)
                            .unwrap()
                            .clone(),
                    )
                } else {
                    None
                },
            ),
        }
    }

    fn generate_button_config(&self, button: LoginButton) -> ButtonConfig {
        match button {
            LoginButton::Confirm => {
                ButtonConfig::new(self.state == LoginState::Confirm, "Confirm".to_string())
            }
            LoginButton::Quit => {
                ButtonConfig::new(self.state == LoginState::Quit, "Quit".to_string())
            }
        }
    }
}

impl State for Login {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        let height = 2 * InputConfig::height() + ButtonConfig::height();
        let width = InputConfig::width();
        let rect = centered_absolute_rect(rect, width, height);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(InputConfig::height()),
                Constraint::Length(InputConfig::height()),
                Constraint::Length(ButtonConfig::height()),
            ])
            .split(rect);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[2]);

        let username_config = self.generate_input_config(LoginInput::Username);
        let master_password_config = self.generate_input_config(LoginInput::MasterPassword);
        let confirm_config = self.generate_button_config(LoginButton::Confirm);
        let quit_config = self.generate_button_config(LoginButton::Quit);
        let mut buffer = f.buffer_mut();

        Input::render(&mut buffer, layout[0], &username_config);
        Input::render(&mut buffer, layout[1], &master_password_config);
        Button::render(&mut buffer, inner_layout[0], &quit_config);
        Button::render(&mut buffer, inner_layout[1], &confirm_config);
    }

    fn handle_key(&mut self, key: &KeyEvent, app: &Application) -> Application {
        let mut app = app.clone();
        let mut change_state = false;

        match self.state {
            LoginState::Username => match key.code {
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = LoginState::MasterPassword;
                }
                KeyCode::Up => {
                    self.state = LoginState::Confirm;
                }
                _ => {
                    let config = self.generate_input_config(LoginInput::Username);
                    let (value, cursor_position) =
                        Input::handle_key(key, &config, self.username.clone());
                    self.username = value;
                    self.cursors.insert(LoginInput::Username, cursor_position);
                }
            },
            LoginState::MasterPassword => match key.code {
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = LoginState::Quit;
                }
                KeyCode::Up => {
                    self.state = LoginState::Username;
                }
                _ => {
                    let config = self.generate_input_config(LoginInput::MasterPassword);
                    let (value, cursor_position) =
                        Input::handle_key(key, &config, self.master_password.clone());
                    self.master_password = value;
                    self.cursors
                        .insert(LoginInput::MasterPassword, cursor_position);
                }
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
                    let res = self.login();
                    match res {
                        Ok((user, ro_records)) => {
                            app.state = ScreenState::Home(Home::new(
                                user,
                                ro_records,
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
