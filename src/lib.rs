mod ui;

mod db;

mod crypto;

use std::path::PathBuf;

pub use ui::start;
pub use db::{init as db_init, create_file};
pub use crypto::hash;
use ui::states::{LoginState, ScreenState};

pub struct Application {
    pub db_path: PathBuf,
    pub state: ScreenState,
}

impl Application {
    pub fn new(db_path: PathBuf) -> Self {
        Application {
            db_path,
            state: ScreenState::Login(LoginState::new()),
        }
    }
}
