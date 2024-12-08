use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::{
    ui::{
        centered_rect,
        states::{login_state::Login, register_state::Register, ScreenState, State},
    },
    Application,
};

#[derive(Clone)]
pub enum StartUpState {
    Login,
    Register,
    Quit,
}

#[derive(Clone)]
pub struct StartUp {
    pub state: StartUpState,
}

impl StartUp {
    pub fn new() -> Self {
        StartUp {
            state: StartUpState::Login,
        }
    }
}

impl State for StartUp {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        let rect = centered_rect(rect, 50, 40);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(rect);

        let text = vec![Line::from(vec!["Login".into()])];
        let login_p = Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(match self.state {
                        StartUpState::Login => Color::White,
                        _ => Color::DarkGray,
                    }))
                    .padding(Padding::new(1, 0, layout[0].height / 4, 0)),
            )
            .style(Style::new().white())
            .alignment(Alignment::Left);

        let text = vec![Line::from(vec!["Register".into()])];
        let register_p = Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(match self.state {
                        StartUpState::Register => Color::White,
                        _ => Color::DarkGray,
                    }))
                    .padding(Padding::new(1, 0, layout[1].height / 4, 0)),
            )
            .style(Style::new().white())
            .alignment(Alignment::Left);

        let text = vec![Line::from(vec!["Quit".into()])];
        let quit_p = Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(match self.state {
                        StartUpState::Quit => Color::White,
                        _ => Color::DarkGray,
                    }))
                    .padding(Padding::new(1, 0, layout[2].height / 4, 0)),
            )
            .style(Style::new().white())
            .alignment(Alignment::Left);

        f.render_widget(login_p, layout[0]);
        f.render_widget(register_p, layout[1]);
        f.render_widget(quit_p, layout[2]);
    }

    fn handle_key(&mut self, key: &KeyEvent, app: &Application) -> Application {
        let mut app = app.clone();
        let mut change_state = false;

        if key.code == KeyCode::Char('q') {
            app.mutable_app_state.running = false;
            return app;
        }

        match self.state {
            StartUpState::Login => match key.code {
                KeyCode::Enter => {
                    app.state = ScreenState::Login(Login::new(&app.immutable_app_state.db_path));
                    change_state = true;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => {
                    self.state = StartUpState::Register;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state = StartUpState::Quit;
                }
                _ => {}
            },
            StartUpState::Register => match key.code {
                KeyCode::Enter => {
                    app.state =
                        ScreenState::Register(Register::new(&app.immutable_app_state.db_path));
                    change_state = true;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => {
                    self.state = StartUpState::Quit;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state = StartUpState::Login;
                }
                _ => {}
            },
            StartUpState::Quit => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.running = false;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Char('j') => {
                    self.state = StartUpState::Login;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.state = StartUpState::Register;
                }
                _ => {}
            },
        }

        if !change_state {
            app.state = ScreenState::StartUp(self.clone());
        }

        app
    }
}
