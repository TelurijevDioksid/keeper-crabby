use std::path::PathBuf;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use krab_backend::user::{RecordOperationConfig, User};

use crate::{
    popups::{
        insert_pwd_popup::{InsertPwd, InsertPwdExitState},
        message_popup::MessagePopup,
        Popup,
    },
    Application,
    {
        centered_rect,
        states::{startup_state::StartUp, ScreenState},
        State,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterState {
    Username,
    MasterPassword,
    ConfirmMasterPassword,
    Confirm,
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Register {
    pub username: String,
    pub master_password: String,
    pub confirm_master_password: String,
    pub state: RegisterState,
    pub domain: String,
    pub pwd: String,
    pub path: PathBuf,
}

impl Register {
    pub fn new(path: &PathBuf) -> Self {
        Register {
            username: String::new(),
            master_password: String::new(),
            confirm_master_password: String::new(),
            state: RegisterState::Username,
            domain: String::new(),
            pwd: String::new(),
            path: path.clone(),
        }
    }

    pub fn username_append(&mut self, c: char) {
        self.username.push(c);
    }

    pub fn master_password_append(&mut self, c: char) {
        self.master_password.push(c);
    }

    pub fn confirm_master_password_append(&mut self, c: char) {
        self.confirm_master_password.push(c);
    }

    pub fn username_pop(&mut self) {
        self.username.pop();
    }

    pub fn master_password_pop(&mut self) {
        self.master_password.pop();
    }

    pub fn confirm_master_password_pop(&mut self) {
        self.confirm_master_password.pop();
    }
}

impl State for Register {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        // need to create input widget
        // this is a temporary solution
        let rect = centered_rect(rect, 50, 40);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(rect);

        let text = vec![Line::from(vec![Span::raw(self.username.clone())])];
        let username_p =
            Paragraph::new(text).block(Block::bordered().title("Username").border_style(
                Style::default().fg(match self.state {
                    RegisterState::Username => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let text = vec![Line::from(vec![Span::raw(self.master_password.clone())])];
        let master_password_p =
            Paragraph::new(text).block(Block::bordered().title("Master Password").border_style(
                Style::default().fg(match self.state {
                    RegisterState::MasterPassword => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let text = vec![Line::from(vec![Span::raw(
            self.confirm_master_password.clone(),
        )])];
        let confirm_master_password_p = Paragraph::new(text).block(
            Block::bordered()
                .title("Confirm Master Password")
                .border_style(Style::default().fg(match self.state {
                    RegisterState::ConfirmMasterPassword => Color::White,
                    _ => Color::DarkGray,
                })),
        );

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[3]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                RegisterState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let register_p = Paragraph::new(Span::raw("Confirm")).block(
            Block::bordered().border_style(Style::default().fg(match self.state {
                RegisterState::Confirm => Color::White,
                _ => Color::DarkGray,
            })),
        );

        f.render_widget(username_p, layout[0]);
        f.render_widget(master_password_p, layout[1]);
        f.render_widget(confirm_master_password_p, layout[2]);
        f.render_widget(quit_p, inner_layout[0]);
        f.render_widget(register_p, inner_layout[1]);
    }

    fn handle_key(&mut self, key: &KeyEvent, app: &Application) -> Application {
        let mut app = app.clone();
        let mut change_state = false;

        match self.state {
            RegisterState::Username => match key.code {
                KeyCode::Char(c) => {
                    self.username_append(c);
                }
                KeyCode::Backspace => {
                    self.username_pop();
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = RegisterState::MasterPassword;
                }
                KeyCode::Up => {
                    self.state = RegisterState::Confirm;
                }
                _ => {}
            },
            RegisterState::MasterPassword => match key.code {
                KeyCode::Char(c) => {
                    self.master_password_append(c);
                }
                KeyCode::Backspace => {
                    self.master_password_pop();
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = RegisterState::ConfirmMasterPassword;
                }
                KeyCode::Up => {
                    self.state = RegisterState::Username;
                }
                _ => {}
            },
            RegisterState::ConfirmMasterPassword => match key.code {
                KeyCode::Char(c) => {
                    self.confirm_master_password_append(c);
                }
                KeyCode::Backspace => {
                    self.confirm_master_password_pop();
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    self.state = RegisterState::Quit;
                }
                KeyCode::Up => {
                    self.state = RegisterState::MasterPassword;
                }
                _ => {}
            },
            RegisterState::Quit => match key.code {
                KeyCode::Enter => {
                    app.state = ScreenState::StartUp(StartUp::new());
                    change_state = true;
                }
                KeyCode::Right | KeyCode::Left | KeyCode::Tab => {
                    self.state = RegisterState::Confirm;
                }
                KeyCode::Up => {
                    self.state = RegisterState::ConfirmMasterPassword;
                }
                KeyCode::Down => {
                    self.state = RegisterState::Username;
                }
                _ => {}
            },
            RegisterState::Confirm => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state
                        .popups
                        .push(Box::new(InsertPwd::new()));
                    change_state = true;
                }
                KeyCode::Right | KeyCode::Left => {
                    self.state = RegisterState::Quit;
                }
                KeyCode::Up => {
                    self.state = RegisterState::ConfirmMasterPassword;
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.state = RegisterState::Username;
                }
                _ => {}
            },
        }

        if !change_state {
            app.state = ScreenState::Register(self.clone());
        }

        app
    }

    fn handle_insert_record_popup(
        &mut self,
        app: Application,
        _popup: Box<dyn Popup>,
    ) -> Application {
        if self.master_password != self.confirm_master_password {
            let mut app = app.clone();
            app.mutable_app_state
                .popups
                .push(Box::new(MessagePopup::new(
                    "Could not create user.".to_string(),
                )));
            return app;
        }

        let domain: String;
        let pwd: String;
        let insert_pwd = _popup.downcast::<InsertPwd>();

        match insert_pwd {
            Ok(insert_pwd) => {
                if insert_pwd.exit_state == Some(InsertPwdExitState::Quit) {
                    return app;
                }
                domain = insert_pwd.domain.clone();
                pwd = insert_pwd.pwd.clone();
            }
            Err(_) => {
                unreachable!();
            }
        }

        let mut app = app.clone();

        let config = RecordOperationConfig::new(
            &self.username,
            &self.master_password,
            &domain,
            &pwd,
            &self.path,
        );

        // first need to validate config
        // match config.validate() ...

        let res = User::new(&config);

        match res {
            Ok(_) => {
                app.state = ScreenState::StartUp(StartUp::new());
            }
            Err(_) => {
                app.mutable_app_state
                    .popups
                    .push(Box::new(MessagePopup::new(
                        "Could not create user.".to_string(),
                    )));
            }
        }

        app
    }
}
