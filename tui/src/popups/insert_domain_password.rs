use krab_backend::generate_password;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::{
    centered_rect,
    popups::{Popup, PopupType},
    Application,
};

/// Represents the state of the insert domain password popup
///
/// # Variants
/// * `Domain` - The domain state
/// * `Password` - The password state
/// * `Confirm` - The confirm state
/// * `Quit` - The quit state
#[derive(Clone)]
pub enum InsertDomainPasswordState {
    Domain,
    Password,
    Confirm,
    Quit,
}

/// Represents the exit state of the insert domain password popup
///
/// # Variants
/// * `Confirm` - The confirm state
/// * `Quit` - The quit state
#[derive(Clone, PartialEq)]
pub enum InsertDomainPasswordExitState {
    Confirm,
    Quit,
}

/// Represents the insert domain password popup
///
/// # Fields
/// * `domain` - The domain
/// * `password` - The password
/// * `state` - The state
/// * `exit_state` - The exit state
/// * `x_percent` - The x percentage
/// * `y_percent` - The y percentage
///
/// # Methods
/// * `new` - Creates a new `InsertDomainPassword`
///
/// # Implements
/// * `Popup` - The popup trait
#[derive(Clone)]
pub struct InsertDomainPassword {
    pub domain: String,
    pub password: String,
    pub state: InsertDomainPasswordState,
    pub exit_state: Option<InsertDomainPasswordExitState>,
    x_percent: u16,
    y_percent: u16,
}

impl InsertDomainPassword {
    /// Creates a new insert domain password popup
    ///
    /// # Returns
    /// A new `InsertDomainPassword`
    pub fn new() -> Self {
        InsertDomainPassword {
            domain: String::new(),
            password: String::new(),
            state: InsertDomainPasswordState::Domain,
            exit_state: None,
            x_percent: 40,
            y_percent: 20,
        }
    }

    /// Appends a character to the domain
    ///
    /// # Arguments
    /// * `c` - The character to append
    pub fn domain_append(&mut self, c: char) {
        self.domain.push(c);
    }

    /// Appends a character to the password
    ///
    /// # Arguments
    /// * `c` - The character to append
    pub fn password_append(&mut self, c: char) {
        self.password.push(c);
    }

    /// Pops a character from the domain
    pub fn domain_pop(&mut self) {
        self.domain.pop();
    }

    /// Pops a character from the password
    pub fn password_pop(&mut self) {
        self.password.pop();
    }
}

impl Popup for InsertDomainPassword {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
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
                InsertDomainPasswordState::Domain => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let text = vec![Line::from(vec![Span::raw(self.password.clone())])];
        let password_p =
            Paragraph::new(text).block(Block::bordered().title("Password").border_style(
                Style::default().fg(match self.state {
                    InsertDomainPasswordState::Password => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[2]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertDomainPasswordState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let confirm_p = Paragraph::new(Span::raw("Confirm")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertDomainPasswordState::Confirm => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        f.render_widget(Clear, rect);
        f.render_widget(domain_p, layout[0]);
        f.render_widget(password_p, layout[1]);
        f.render_widget(quit_p, inner_layout[0]);
        f.render_widget(confirm_p, inner_layout[1]);
    }

    fn handle_key(
        &mut self,
        key: &KeyEvent,
        app: &Application,
    ) -> (Application, Option<Box<dyn Popup>>) {
        let mut app = app.clone();
        let mut poped = false;

        match self.state {
            InsertDomainPasswordState::Domain => match key.code {
                KeyCode::Char(c) => {
                    self.domain_append(c);
                }
                KeyCode::Backspace => {
                    self.domain_pop();
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Quit;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    self.state = InsertDomainPasswordState::Password;
                }
                _ => {}
            },
            InsertDomainPasswordState::Password => match key.code {
                KeyCode::Char('g') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        self.password = generate_password();
                    } else {
                        self.password_append('g');
                    }
                }
                KeyCode::Char(c) => {
                    self.password_append(c);
                }
                KeyCode::Backspace => {
                    self.password_pop();
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Domain;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter => {
                    self.state = InsertDomainPasswordState::Quit;
                }
                _ => {}
            },
            InsertDomainPasswordState::Quit => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertDomainPasswordExitState::Quit);
                    poped = true;
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Password;
                }
                KeyCode::Right | KeyCode::Tab | KeyCode::Left => {
                    self.state = InsertDomainPasswordState::Confirm;
                }
                KeyCode::Down => {
                    self.state = InsertDomainPasswordState::Domain;
                }
                _ => {}
            },
            InsertDomainPasswordState::Confirm => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertDomainPasswordExitState::Confirm);
                    poped = true;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.state = InsertDomainPasswordState::Quit;
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.state = InsertDomainPasswordState::Domain;
                }
                KeyCode::Up => {
                    self.state = InsertDomainPasswordState::Password;
                }
                _ => {}
            },
        }

        if !poped {
            app.mutable_app_state.popups.pop();
            app.mutable_app_state.popups.push(Box::new(self.clone()));
            return (app, None);
        }

        (app, Some(Box::new(self.clone())))
    }

    fn wrapper(&self, rect: Rect) -> Rect {
        centered_rect(rect, self.x_percent, self.y_percent)
    }

    fn popup_type(&self) -> PopupType {
        PopupType::InsertDomainPassword
    }
}
