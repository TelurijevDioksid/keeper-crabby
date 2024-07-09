use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};
use std::error::Error;
use std::io;
use std::path::PathBuf;

use crate::Application;

pub mod states;

pub fn ui(f: &mut Frame, _app: &Application) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Press 'q' to quit",
        Style::default().fg(Color::White),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    match &_app.state {
        states::ScreenState::Login(state) => {
            let login_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let login = Paragraph::new(Text::styled(
                &state.username,
                Style::default().fg(Color::White),
            ))
            .block(login_block);

            f.render_widget(login, chunks[1]);
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Application) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match &mut app.state {
                states::ScreenState::Login(state) => match key.code {
                    event::KeyCode::Char('q') => {
                        break Ok(false);
                    }
                    event::KeyCode::Char(value) => {
                        state.username_append(value);
                    }
                    _ => {}
                },
            }
        }
    }
}

pub fn start(db_path: PathBuf) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let beckend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(beckend)?;

    let mut app = Application::new(db_path);
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
