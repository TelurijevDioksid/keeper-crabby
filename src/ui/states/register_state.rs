use std::path::PathBuf;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    crypto::user::{RecordOperationConfig, User},
    ui::{
        popups::insert_pwd_popup::{DomainPwdInsert, InsertPwd},
        {
            centered_rect,
            states::{startup_state::StartUp, ScreenState},
            State,
        },
    },
    ImutableAppState, MutableAppState,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterState {
    Username,
    MasterPassword,
    ConfirmMasterPassword,
    Domain,
    Pwd,
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

    pub fn copy_with_state(&self, state: RegisterState) -> Self {
        Register {
            username: self.username.clone(),
            master_password: self.master_password.clone(),
            confirm_master_password: self.confirm_master_password.clone(),
            state,
            domain: self.domain.clone(),
            pwd: self.pwd.clone(),
            path: self.path.clone(),
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

impl DomainPwdInsert for Register {
    fn set_pwd(&self, pwd: String) -> Register {
        let mut ss = self.clone();
        ss.pwd = pwd;
        ss
    }

    fn set_domain(&self, domain: String) -> Register {
        let mut ss = self.clone();
        ss.domain = domain;
        ss
    }

    fn confirm(&self) -> Result<(), String> {
        if self.master_password != self.confirm_master_password {
            return Err("Could not create user.".to_string());
        }

        let config = RecordOperationConfig::new(
            &self.username,
            &self.master_password,
            &self.domain,
            &self.pwd,
            &self.path,
        );

        // first need to validate config
        // match config.validate() ...

        let res = User::new(&config);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl State for Register {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
    ) {
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

    fn handle_key(
        &mut self,
        key: KeyEvent,
        _immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState) {
        let mut mutable_state = mutable_state.clone();
        let mut screen_state = ScreenState::Register(self.clone());

        match self.state {
            RegisterState::Username => match key.code {
                KeyCode::Char(c) => {
                    let mut ss = self.copy_with_state(RegisterState::Username);
                    ss.username_append(c);
                    screen_state = ScreenState::Register(ss);
                }
                KeyCode::Backspace => {
                    let mut ss = self.copy_with_state(RegisterState::Username);
                    ss.username_pop();
                    screen_state = ScreenState::Register(ss);
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::MasterPassword));
                }
                KeyCode::Up => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::Confirm));
                }
                _ => {}
            },
            RegisterState::MasterPassword => match key.code {
                KeyCode::Char(c) => {
                    let mut ss = self.copy_with_state(RegisterState::MasterPassword);
                    ss.master_password_append(c);
                    screen_state = ScreenState::Register(ss);
                }
                KeyCode::Backspace => {
                    let mut ss = self.copy_with_state(RegisterState::MasterPassword);
                    ss.master_password_pop();
                    screen_state = ScreenState::Register(ss);
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    screen_state = ScreenState::Register(
                        self.copy_with_state(RegisterState::ConfirmMasterPassword),
                    );
                }
                KeyCode::Up => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::Username));
                }
                _ => {}
            },
            RegisterState::ConfirmMasterPassword => match key.code {
                KeyCode::Char(c) => {
                    let mut ss = self.copy_with_state(RegisterState::ConfirmMasterPassword);
                    ss.confirm_master_password_append(c);
                    screen_state = ScreenState::Register(ss);
                }
                KeyCode::Backspace => {
                    let mut ss = self.copy_with_state(RegisterState::ConfirmMasterPassword);
                    ss.confirm_master_password_pop();
                    screen_state = ScreenState::Register(ss);
                }
                KeyCode::Enter | KeyCode::Tab | KeyCode::Down => {
                    screen_state = ScreenState::Register(self.copy_with_state(RegisterState::Quit));
                }
                KeyCode::Up => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::MasterPassword));
                }
                _ => {}
            },
            RegisterState::Quit => match key.code {
                KeyCode::Enter => {
                    screen_state = ScreenState::StartUp(StartUp::new());
                }
                KeyCode::Right | KeyCode::Left | KeyCode::Tab => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::Confirm));
                }
                KeyCode::Up => {
                    screen_state = ScreenState::Register(
                        self.copy_with_state(RegisterState::ConfirmMasterPassword),
                    );
                }
                KeyCode::Down => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::Username));
                }
                _ => {}
            },
            RegisterState::Confirm => match key.code {
                KeyCode::Enter => {
                    mutable_state.popups.push(Box::new(InsertPwd::new()));
                }
                KeyCode::Right | KeyCode::Left => {
                    screen_state = ScreenState::Register(self.copy_with_state(RegisterState::Quit));
                }
                KeyCode::Up => {
                    screen_state = ScreenState::Register(
                        self.copy_with_state(RegisterState::ConfirmMasterPassword),
                    );
                }
                KeyCode::Down | KeyCode::Tab => {
                    screen_state =
                        ScreenState::Register(self.copy_with_state(RegisterState::Username));
                }
                _ => {}
            },
            _ => {}
        }

        (mutable_state, screen_state)
    }
}
