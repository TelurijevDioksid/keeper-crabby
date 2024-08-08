use crate::{
    ui::{login_state::Login, startup_state::StartUp},
    ImutableAppState, MutableAppState,
};
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use super::register_state::Register;

#[derive(Clone)]
pub enum ScreenState {
    Login(Login),
    StartUp(StartUp),
    Register(Register),
}

pub trait State {
    fn render(
        &self,
        f: &mut Frame,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        rect: Rect,
    );
    fn handle_key(
        &mut self,
        key: KeyEvent,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState);
}
