use dyn_clone::DynClone;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{ui::states::ScreenState, ImutableAppState, MutableAppState};

pub trait Popup: DynClone {
    fn render(
        &self,
        f: &mut Frame,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        rect: Rect,
        current_state: &ScreenState,
    );
    fn handle_key(
        &mut self,
        immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        key: &KeyEvent,
        previous_state: &ScreenState,
    ) -> (MutableAppState, Option<ScreenState>);

    fn wrapper(&self, rect: Rect) -> Rect;
}

dyn_clone::clone_trait_object!(Popup);
