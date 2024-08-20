use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::{
    ui::{
        centered_rect, message_popup::MessagePopup, popup::Popup, startup_state::StartUp,
        states::ScreenState,
    },
    ImutableAppState, MutableAppState,
};

pub trait DomainPwdInsert {
    fn set_domain(&self, domain: String) -> Self;
    fn set_pwd(&self, pwd: String) -> Self;
    fn confirm(&self) -> Result<(), String>;
}

#[derive(Clone)]
pub enum InsertPwdState {
    Domain,
    Pwd,
    Confirm,
    Quit,
}

#[derive(Clone)]
pub struct InsertPwd {
    pub domain: String,
    pub pwd: String,
    pub state: InsertPwdState,
    x_percent: u16,
    y_percent: u16,
}

impl InsertPwd {
    pub fn new() -> Self {
        InsertPwd {
            domain: String::new(),
            pwd: String::new(),
            state: InsertPwdState::Domain,
            x_percent: 40,
            y_percent: 20,
        }
    }

    pub fn copy_with_state(&self, state: InsertPwdState) -> Self {
        InsertPwd {
            domain: self.domain.clone(),
            pwd: self.pwd.clone(),
            state,
            x_percent: self.x_percent,
            y_percent: self.y_percent,
        }
    }

    pub fn domain_append(&mut self, c: char) {
        self.domain.push(c);
    }

    pub fn pwd_append(&mut self, c: char) {
        self.pwd.push(c);
    }

    pub fn domain_pop(&mut self) {
        self.domain.pop();
    }

    pub fn pwd_pop(&mut self) {
        self.pwd.pop();
    }
}

impl Popup for InsertPwd {
    fn render(
        &self,
        f: &mut Frame,
        _immutable_state: &ImutableAppState,
        _mutable_state: &MutableAppState,
        rect: Rect,
        _current_state: &ScreenState,
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Min(0),
                Constraint::Min(0),
            ])
            .split(rect);

        let text = vec![Line::from(vec![Span::raw(self.domain.clone())])];
        let domain_p = Paragraph::new(text).block(Block::bordered().title("Domain").border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Domain => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let text = vec![Line::from(vec![Span::raw(self.pwd.clone())])];
        let pwd_p = Paragraph::new(text).block(Block::bordered().title("Password").border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Pwd => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[2]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let confirm_p = Paragraph::new(Span::raw("Confirm")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertPwdState::Confirm => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        f.render_widget(Clear, rect);
        f.render_widget(domain_p, layout[0]);
        f.render_widget(pwd_p, layout[1]);
        f.render_widget(quit_p, inner_layout[0]);
        f.render_widget(confirm_p, inner_layout[1]);
    }

    fn handle_key(
        &mut self,
        _immutable_state: &ImutableAppState,
        mutable_state: &MutableAppState,
        key: &KeyEvent,
        previous_state: &ScreenState,
    ) -> (MutableAppState, Option<ScreenState>) {
        let mut mutable_state = mutable_state.clone();
        let mut screen_state = previous_state.clone();
        match self.state {
            InsertPwdState::Domain => match key.code {
                KeyCode::Char(c) => {
                    self.domain_append(c);
                    match previous_state {
                        ScreenState::Register(s) => {
                            let state = s.set_domain(self.domain.clone());
                            let insert_pwd = self.clone();
                            mutable_state.popups.pop();
                            mutable_state.popups.push(Box::new(insert_pwd));
                            screen_state = ScreenState::Register(state);
                        }
                        _ => {}
                    }
                }
                KeyCode::Backspace => {
                    self.domain_pop();
                    match previous_state {
                        ScreenState::Register(s) => {
                            let state = s.set_domain(self.domain.clone());
                            let insert_pwd = self.clone();
                            mutable_state.popups.pop();
                            mutable_state.popups.push(Box::new(insert_pwd));
                            screen_state = ScreenState::Register(state);
                        }
                        _ => {}
                    }
                }
                KeyCode::Up => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Confirm);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Pwd);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                _ => {}
            },
            InsertPwdState::Pwd => match key.code {
                KeyCode::Char(c) => {
                    self.pwd_append(c);
                    match previous_state {
                        ScreenState::Register(s) => {
                            let state = s.set_pwd(self.pwd.clone());
                            let insert_pwd = self.clone();
                            mutable_state.popups.pop();
                            mutable_state.popups.push(Box::new(insert_pwd));
                            screen_state = ScreenState::Register(state);
                        }
                        _ => {}
                    }
                }
                KeyCode::Backspace => {
                    self.pwd_pop();
                    match previous_state {
                        ScreenState::Register(s) => {
                            let state = s.set_pwd(self.pwd.clone());
                            let insert_pwd = self.clone();
                            mutable_state.popups.pop();
                            mutable_state.popups.push(Box::new(insert_pwd));
                            screen_state = ScreenState::Register(state);
                        }
                        _ => {}
                    }
                }
                KeyCode::Up => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Domain);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Quit);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                _ => {}
            },
            InsertPwdState::Quit => match key.code {
                KeyCode::Enter => {
                    mutable_state.popups.pop();
                }
                KeyCode::Up => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Pwd);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                KeyCode::Right | KeyCode::Tab | KeyCode::Left => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Confirm);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                KeyCode::Down => {
                    let insert_pwd = self.copy_with_state(InsertPwdState::Domain);
                    mutable_state.popups.pop();
                    mutable_state.popups.push(Box::new(insert_pwd));
                }
                _ => {}
            },
            InsertPwdState::Confirm => {
                match key.code {
                    KeyCode::Enter => match previous_state {
                        ScreenState::Register(s) => match s.confirm() {
                            Ok(_) => {
                                mutable_state.popups.pop();
                                screen_state = ScreenState::StartUp(StartUp::new());
                            }
                            Err(e) => {
                                mutable_state.popups.push(Box::new(MessagePopup::new(e, |_immutable_state: &ImutableAppState<'_>, mutable_state: &MutableAppState, _screen_state: &ScreenState| {
                                let mut mutable_state = mutable_state.clone();
                                mutable_state.popups.pop();
                                (mutable_state, None)
                            }
                                )));
                            }
                        },
                        _ => {}
                    },
                    KeyCode::Left | KeyCode::Right => {
                        let insert_pwd = self.copy_with_state(InsertPwdState::Quit);
                        mutable_state.popups.pop();
                        mutable_state.popups.push(Box::new(insert_pwd));
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        let insert_pwd = self.copy_with_state(InsertPwdState::Domain);
                        mutable_state.popups.pop();
                        mutable_state.popups.push(Box::new(insert_pwd));
                    }
                    KeyCode::Up => {
                        let insert_pwd = self.copy_with_state(InsertPwdState::Pwd);
                        mutable_state.popups.pop();
                        mutable_state.popups.push(Box::new(insert_pwd));
                    }
                    _ => {}
                }
            }
        }

        (mutable_state, Some(screen_state))
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, self.x_percent, self.y_percent)
    }
}
