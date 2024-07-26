use dyn_clone::DynClone;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{ImutableAppState, MutableAppState};

pub trait Popup: DynClone {
    fn render(
        &self,
        f: &mut Frame,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        rect: Rect,
    );
    fn handle_key(
        &mut self,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        key: &KeyEvent,
    ) -> MutableAppState;
}

dyn_clone::clone_trait_object!(Popup);
