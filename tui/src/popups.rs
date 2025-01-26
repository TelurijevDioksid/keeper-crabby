use downcast_rs::{impl_downcast, Downcast};

use dyn_clone::DynClone;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::Application;

pub mod exit;
pub mod insert_domain_password;
pub mod insert_master;
pub mod message;

/// Represents the type of a popup
///
/// # Variants
/// * `Exit` - The exit popup
/// * `InsertDomainPassword` - The insert domain password popup
/// * `Message` - The message popup
/// * `InsertMaster` - The insert master password popup
pub enum PopupType {
    Exit,
    InsertDomainPassword,
    Message,
    InsertMaster,
}

/// Represents a popup
///
/// # Traits
/// * `DynClone`
/// * `Downcast`
///
/// # Methods
/// * `render` - Renders the popup
/// * `handle_key` - Handles a key event
/// * `wrapper` - Wraps the popup in a rectangle
/// * `popup_type` - Returns the type of the popup
pub trait Popup: DynClone + Downcast {
    /// Renders the popup
    ///
    /// # Arguments
    /// * `f` - The frame
    /// * `app` - The application
    /// * `rect` - The rectangle to render the popup in
    fn render(&self, f: &mut Frame, app: &Application, rect: Rect);

    /// Handles a key event
    ///
    /// # Arguments
    /// * `key` - The key event
    /// * `app` - The application
    ///
    /// # Returns
    /// A tuple containing the updated application and an optional popup
    fn handle_key(
        &mut self,
        key: &KeyEvent,
        app: &Application,
    ) -> (Application, Option<Box<dyn Popup>>);

    /// Wraps the popup in a rectangle
    ///
    /// # Arguments
    /// * `rect` - The rectangle to wrap the popup in
    ///
    /// # Returns
    /// The wrapped rectangle
    fn wrapper(&self, rect: Rect) -> Rect;

    /// Returns the type of the popup
    ///
    /// # Returns
    /// The type of the popup
    fn popup_type(&self) -> PopupType;
}

dyn_clone::clone_trait_object!(Popup);

impl_downcast!(Popup);
