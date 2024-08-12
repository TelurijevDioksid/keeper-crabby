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
        states::{ScreenState, State},
        login_state::Login,
        register_state::Register,
    },
    ImutableAppState, MutableAppState,
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

    pub fn new_with_state(state: StartUpState) -> Self {
        StartUp { state }
    }
}

impl State for StartUp {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
    ) {
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

    fn handle_key(
        &mut self,
        key: KeyEvent,
        _immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
    ) -> (MutableAppState, ScreenState) {
        let mut mutable_state = mutable_state.clone();
        let mut screen_state = ScreenState::StartUp(self.clone());

        match key.code {
            KeyCode::Char('q') => {
                mutable_state.running = false;
            }
            KeyCode::Char('j') | KeyCode::Down => match self.state {
                StartUpState::Login => {
                    screen_state =
                        ScreenState::StartUp(StartUp::new_with_state(StartUpState::Register));
                }
                StartUpState::Register => {
                    screen_state =
                        ScreenState::StartUp(StartUp::new_with_state(StartUpState::Quit));
                }
                StartUpState::Quit => {
                    screen_state =
                        ScreenState::StartUp(StartUp::new_with_state(StartUpState::Login));
                }
            },
            KeyCode::Char('k') | KeyCode::Up => match self.state {
                StartUpState::Login => {
                    screen_state =
                        ScreenState::StartUp(StartUp::new_with_state(StartUpState::Quit));
                }
                StartUpState::Register => {
                    screen_state =
                        ScreenState::StartUp(StartUp::new_with_state(StartUpState::Login));
                }
                StartUpState::Quit => {
                    screen_state =
                        ScreenState::StartUp(StartUp::new_with_state(StartUpState::Register));
                }
            },
            KeyCode::Enter => match self.state {
                StartUpState::Login => {
                    screen_state = ScreenState::Login(Login::new());
                }
                StartUpState::Register => {
                    screen_state = ScreenState::Register(Register::new());
                }
                StartUpState::Quit => {
                    mutable_state.running = false;
                }
            },
            _ => {}
        }

        (mutable_state, screen_state)
    }
}
