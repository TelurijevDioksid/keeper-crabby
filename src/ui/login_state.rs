use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    ui::{
        exit_popup::Exit,
        states::{ScreenState, State},
    },
    ImutableAppState, MutableAppState,
};

#[derive(Clone)]
pub enum LoginState {
    Username,
    MasterPassword,
}

#[derive(Clone)]
pub struct Login {
    pub username: String,
    pub master_password: String,
    pub state: LoginState,
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

    pub fn new() -> Self {
        Login {
            username: String::new(),
            master_password: String::new(),
            state: LoginState::Username,
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
        let wrapper = Block::default().borders(Borders::ALL).title("Login");
        f.render_widget(wrapper, rect);
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        _immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState) {
        let mut mutable_state = mutable_state.clone();
        let screen_state = ScreenState::Login(self.clone());
        match self.state {
            LoginState::Username => match key.code {
                KeyCode::Enter => {
                    self.state = LoginState::MasterPassword;
                }
                KeyCode::Backspace => {
                    self.username_pop();
                }
                KeyCode::Char(value) => {
                    self.username_append(value);
                }
                KeyCode::Esc => {
                    mutable_state.popups.push(Box::new(Exit::new()));
                }
                _ => {}
            },
            LoginState::MasterPassword => match key.code {
                KeyCode::Enter => {
                    self.state = LoginState::Username;
                }
                KeyCode::Backspace => {
                    self.master_password_pop();
                }
                KeyCode::Char(value) => {
                    self.master_password_append(value);
                }
                KeyCode::Esc => {
                    mutable_state.popups.push(Box::new(Exit::new()));
                }
                _ => {}
            },
        }
        (mutable_state, screen_state)
    }
}
