use std::{cell::RefCell, error::Error, io, path::PathBuf};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame, Terminal,
};

use crate::{
    popups::{Popup, PopupType},
    states::{startup_state::StartUp, ScreenState, State},
};

pub mod components;
pub mod popups;
pub mod states;

pub fn ui(f: &mut Frame, app: &Application) {
    let wrapper = Rect::new(0, 0, f.area().width, f.area().height);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(app.immutable_app_state.name.clone()),
        wrapper,
    );
    let rect = centered_rect(f.area(), 97, 94);
    match &app.state {
        ScreenState::Login(s) => s.render(f, app, rect),
        ScreenState::StartUp(s) => {
            s.render(f, app, rect);
        }
        ScreenState::Register(s) => {
            s.render(f, app, rect);
        }
        ScreenState::Home(s) => s.render(f, app, rect),
    }
    for popup in &app.mutable_app_state.popups {
        popup.render(f, app, popup.wrapper(rect));
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    application: RefCell<Application>,
) -> io::Result<bool> {
    loop {
        let app = application.borrow();
        let should_break = !app.mutable_app_state.running;

        if should_break {
            break;
        }

        let _ = terminal.draw(|f| ui(f, &app));
        drop(app);

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            let app = application.borrow();
            let app_copy = app.clone();
            let amount_of_popups = app_copy.mutable_app_state.popups.len();
            drop(app);
            if amount_of_popups > 0 {
                let mut app = application.borrow_mut();
                let (changed_app, last_state) =
                    app.mutable_app_state.popups[amount_of_popups - 1].handle_key(&key, &app_copy);
                app.mutable_app_state = changed_app.mutable_app_state;
                app.state = changed_app.state;

                if let Some(last_state) = last_state {
                    let mut new_app: Application = app.clone();
                    match last_state.popup_type() {
                        PopupType::InsertPwd => match &mut app.state {
                            ScreenState::Register(s) => {
                                new_app = s.handle_insert_record_popup(new_app, last_state);
                            }
                            _ => {}
                        },
                        _ => {}
                    }

                    app.mutable_app_state = new_app.mutable_app_state;
                    app.state = new_app.state;
                }
            } else {
                let mut app = application.borrow_mut();
                let changed_app: Application;
                match &mut app.state {
                    ScreenState::Login(s) => changed_app = s.handle_key(&key, &app_copy),
                    ScreenState::StartUp(s) => changed_app = s.handle_key(&key, &app_copy),
                    ScreenState::Home(s) => changed_app = s.handle_key(&key, &app_copy),
                    ScreenState::Register(s) => changed_app = s.handle_key(&key, &app_copy),
                };

                app.mutable_app_state = changed_app.mutable_app_state;
                app.state = changed_app.state;
            }
        }
        let mut app = application.borrow_mut();
        app.immutable_app_state.rect = Some(terminal.get_frame().area());
    }
    Ok(true)
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn start(db_path: PathBuf) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let beckend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(beckend)?;

    let rect = terminal.get_frame().area();
    let app = Application::create(db_path, rect);
    let _res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

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
