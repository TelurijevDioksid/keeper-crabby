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
use std::{error::Error, io, path::PathBuf};

use crate::{Application, ImutableAppState, MutableAppState};
use states::{ScreenState, State};

pub mod popup;
pub mod states;

pub mod exit_popup;
pub mod insert_pwd_popup;
pub mod message_popup;

pub mod login_state;
pub mod register_state;
pub mod startup_state;

pub fn ui(
    f: &mut Frame,
    immutable_state: &ImutableAppState,
    mutable_state: &MutableAppState,
    curr_state: &ScreenState,
) {
    let wrapper = Rect::new(0, 0, f.size().width, f.size().height);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(immutable_state.name),
        wrapper,
    );
    let rect = centered_rect(f.size(), 97, 94);
    match &curr_state {
        ScreenState::Login(s) => s.render(f, immutable_state, mutable_state, rect),
        ScreenState::StartUp(s) => s.render(f, immutable_state, mutable_state, rect),
        ScreenState::Register(s) => s.render(f, immutable_state, mutable_state, rect),
    }
    for popup in &mutable_state.popups {
        popup.render(
            f,
            immutable_state,
            mutable_state,
            popup.wrapper(rect),
            curr_state,
        );
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    immutable_state: &ImutableAppState,
    mutable_state: &mut MutableAppState,
    curr_state: &mut ScreenState,
) -> io::Result<bool> {
    let mut ms_curr = mutable_state.clone();
    let mut cs_curr = curr_state.clone();
    loop {
        let should_break = !ms_curr.running;

        if should_break {
            break;
        }

        terminal.draw(|f| ui(f, immutable_state, &ms_curr, &cs_curr))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            let ms: MutableAppState;
            let mut cs = cs_curr.clone();
            let amount_of_popups = ms_curr.popups.len();
            if amount_of_popups > 0 {
                let maybe_cs: Option<ScreenState>;
                let mscopy = ms_curr.clone();
                let last_popup = ms_curr.popups.len() - 1;
                (ms, maybe_cs) =
                    ms_curr.popups[last_popup].handle_key(immutable_state, &mscopy, &key, &cs_curr);
                if maybe_cs.is_some() {
                    cs = maybe_cs.unwrap();
                }
            } else {
                match &mut cs_curr {
                    states::ScreenState::Login(s) => {
                        (ms, cs) = s.handle_key(key, immutable_state, &ms_curr);
                    }
                    states::ScreenState::StartUp(s) => {
                        (ms, cs) = s.handle_key(key, immutable_state, &ms_curr);
                    }
                    states::ScreenState::Register(s) => {
                        (ms, cs) = s.handle_key(key, immutable_state, &ms_curr);
                    }
                }
            }
            ms_curr = ms;
            cs_curr = cs;
        }
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

    let (imutable_app_state, mut mutable_app_state, mut state) = Application::create(db_path);
    let _res = run_app(
        &mut terminal,
        &imutable_app_state,
        &mut mutable_app_state,
        &mut state,
    );

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
