use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    popups::Popup,
    states::{
        home_state::Home, login_state::Login, register_state::Register, startup_state::StartUp,
    },
    Application,
};

pub mod home_state;
pub mod login_state;
pub mod register_state;
pub mod startup_state;

#[derive(Clone)]
pub enum ScreenState {
    Login(Login),
    StartUp(StartUp),
    Register(Register),
    Home(Home),
}

pub trait State {
    fn render(&self, f: &mut Frame, app: &Application, rect: Rect);
    fn handle_key(&mut self, key: &KeyEvent, app: &Application) -> Application;

    fn handle_insert_record_popup(
        &mut self,
        _app: Application,
        _popup: Box<dyn Popup>,
    ) -> Application {
        unreachable!("This state does not handle insert record popups");
    }

    fn handle_insert_master_popup(
        &mut self,
        _app: Application,
        _popup: Box<dyn Popup>,
    ) -> Application {
        unreachable!("This state does not handle insert master popups");
    }
}
