use crate::{
    ui::{login_state::Login, startup_state::StartUp},
    ImutableAppState, MutableAppState,
};
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

#[derive(Clone)]
pub enum ScreenState {
    Login(Login),
    StartUp(StartUp),
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
