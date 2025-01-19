use downcast_rs::{impl_downcast, Downcast};

use dyn_clone::DynClone;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::Application;

pub mod exit;
pub mod insert_domain_password;
pub mod insert_master;
pub mod message;

pub enum PopupType {
    Exit,
    InsertDomainPassword,
    Message,
    InsertMaster,
}

pub trait Popup: DynClone + Downcast {
    fn render(&self, f: &mut Frame, app: &Application, rect: Rect);
    fn handle_key(
        &mut self,
        key: &KeyEvent,
        app: &Application,
    ) -> (Application, Option<Box<dyn Popup>>);

    fn wrapper(&self, rect: Rect) -> Rect;

    fn popup_type(&self) -> PopupType;
}

dyn_clone::clone_trait_object!(Popup);

impl_downcast!(Popup);
