use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    popups::Popup,
    views::{home::Home, login::Login, register::Register, startup::StartUp},
    Application,
};

pub mod home;
pub mod login;
pub mod register;
pub mod startup;

/// Represents the state of the application
///
/// # Variants
/// * `Login` - The login state
/// * `StartUp` - The startup state
/// * `Register` - The register state
/// * `Home` - The home state
#[derive(Clone)]
pub enum ViewState {
    Login(Login),
    StartUp(StartUp),
    Register(Register),
    Home(Home),
}

/// Represents a state
///
/// # Methods
/// * `render` - Renders the state
/// * `handle_key` - Handles a key event
/// * `handle_insert_record_popup` - Handles an insert record popup
/// * `handle_insert_master_popup` - Handles an insert master popup
pub trait View {
    /// Renders the state
    ///
    /// # Arguments
    /// * `f` - The mutable reference to the frame
    /// * `app` - The application
    /// * `rect` - The rectangle to render the state in
    fn render(&self, f: &mut Frame, app: &Application, rect: Rect);

    /// Handles a key event
    ///
    /// # Arguments
    /// * `key` - The key event
    /// * `app` - The application
    ///
    /// # Returns
    /// The updated application
    fn handle_key(&mut self, key: &KeyEvent, app: &Application) -> Application;

    /// Handles an insert record popup
    ///
    /// # Arguments
    /// * `app` - The application
    /// * `popup` - The insert record popup
    ///
    /// # Returns
    /// The updated application
    ///
    /// # Panics
    /// This function panics if called on a state that does not handle insert record popups
    fn handle_insert_record_popup(
        &mut self,
        _app: Application,
        _popup: Box<dyn Popup>,
    ) -> Application {
        unreachable!("This view does not handle insert record popups");
    }

    /// Handles an insert master popup
    ///
    /// # Arguments
    /// * `app` - The application
    /// * `popup` - The insert master popup
    ///
    /// # Returns
    /// The updated application
    ///
    /// # Panics
    /// This function panics if called on a state that does not handle insert master popups
    fn handle_insert_master_popup(
        &mut self,
        _app: Application,
        _popup: Box<dyn Popup>,
    ) -> Application {
        unreachable!("This view does not handle insert master popups");
    }
}
