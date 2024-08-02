use std::path::PathBuf;
use ui::{popup::Popup, startup_state::StartUp, states::ScreenState};

mod crypto;
mod db;
mod ui;

pub use crypto::hash;
pub use db::{create_file, init as db_init};
pub use ui::start;

pub struct Application {}

pub struct ImutableAppState<'a> {
    pub name: &'a str,
    pub db_path: PathBuf,
}

#[derive(Clone)]
pub struct MutableAppState {
    pub popups: Vec<Box<dyn Popup>>,
    pub running: bool,
}

impl Application {
    fn create(db_path: PathBuf) -> (ImutableAppState<'static>, MutableAppState, ScreenState) {
        let imutable_app_state = ImutableAppState {
            name: "Keeper Crabby",
            db_path,
        };

        let mutable_app_state = MutableAppState {
            popups: Vec::new(),
            running: true,
        };

        let state = ScreenState::StartUp(StartUp::new());
        (imutable_app_state, mutable_app_state, state)
    }
}
