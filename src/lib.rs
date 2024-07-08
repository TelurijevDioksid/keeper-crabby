mod ui;

mod db;

pub use ui::start;
use ui::states::{LoginState, ScreenState};

pub struct Application {
    pub state: ScreenState,
    pub username: String,
    pub master_password: String,
}

impl Application {
    pub fn new() -> Self {
        Application {
            state: ScreenState::Login(LoginState::new()),
            username: String::new(),
            master_password: String::new(),
        }
    }
}
