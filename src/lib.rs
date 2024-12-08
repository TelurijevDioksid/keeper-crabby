use ratatui::prelude::Rect;
use std::{cell::RefCell, path::PathBuf};

use ui::{
    popups::Popup,
    states::{startup_state::StartUp, ScreenState},
};

mod crypto;
mod db;
mod ui;

pub use crypto::hash;
pub use db::{clear_file_content, create_file, init as db_init};
pub use ui::start;

#[derive(Clone)]
pub struct Application {
    immutable_app_state: ImmutableAppState,
    mutable_app_state: MutableAppState,
    state: ScreenState,
}

#[derive(Debug, Clone, PartialEq)]
struct ImmutableAppState {
    pub name: String,
    pub db_path: PathBuf,
    pub rect: Option<Rect>,
}

#[derive(Clone)]
struct MutableAppState {
    pub popups: Vec<Box<dyn Popup>>,
    pub running: bool,
}

impl Application {
    fn create(db_path: PathBuf, rect: Rect) -> RefCell<Self> {
        let immutable_app_state = ImmutableAppState {
            name: "Keeper Crabby".to_string(),
            db_path,
            rect: Some(rect),
        };

        let mutable_app_state = MutableAppState {
            popups: Vec::new(),
            running: true,
        };

        let state = ScreenState::StartUp(StartUp::new());
        RefCell::new(Self {
            immutable_app_state,
            mutable_app_state,
            state,
        })
    }
}
