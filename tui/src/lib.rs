use std::{cell::RefCell, error::Error, io, path::PathBuf};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame, Terminal,
};

use crate::{
    popups::{Popup, PopupType},
    states::{startup::StartUp, ScreenState, State},
};

pub mod components;
pub mod popups;
pub mod states;

const COLOR_BLACK: &str = "#503D2D";
const COLOR_CYAN: &str = "#1F9295";
const COLOR_WHITE: &str = "#F0ECC9";
const COLOR_ORANGE: &str = "#E3AD43";
const COLOR_RED: &str = "#D44C1A";

pub fn from(hex: &str) -> Result<Color, String> {
    let hex = hex.trim_start_matches('#');
    let try_r = u8::from_str_radix(&hex[0..2], 16);
    let try_g = u8::from_str_radix(&hex[2..4], 16);
    let try_b = u8::from_str_radix(&hex[4..6], 16);
    if try_r.is_err() || try_g.is_err() || try_b.is_err() {
        return Err("Invalid color".to_string());
    }
    Ok(Color::Rgb(try_r.unwrap(), try_g.unwrap(), try_b.unwrap()))
}

fn ui(f: &mut Frame, app: &Application) {
    let wrapper = Rect::new(0, 0, f.area().width, f.area().height);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" ".to_string() + &app.immutable_app_state.name + " ")
            .style(Style::default().fg(from(COLOR_ORANGE).unwrap_or(Color::Yellow))),
        wrapper,
    );
    let rect = centered_absolute_rect(wrapper, f.area().width - 6, f.area().height - 4);
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
                        PopupType::InsertDomainPassword => match &mut app.state {
                            ScreenState::Register(s) => {
                                new_app = s.handle_insert_record_popup(new_app, last_state);
                            }
                            ScreenState::Home(s) => {
                                new_app = s.handle_insert_record_popup(new_app, last_state);
                            }
                            _ => {}
                        },
                        PopupType::InsertMaster => match &mut app.state {
                            ScreenState::Home(s) => {
                                new_app = s.handle_insert_master_popup(new_app, last_state);
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

fn centered_absolute_rect(r: Rect, width: u16, height: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - height) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height - height) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width - width) / 2),
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
            name: "krab".to_string(),
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
