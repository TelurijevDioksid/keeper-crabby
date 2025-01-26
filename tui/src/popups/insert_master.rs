use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
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

/// Represents the state of the insert master popup
///
/// # Variants
/// * `Master` - The master state
/// * `Confirm` - The confirm state
/// * `Quit` - The quit state
#[derive(Clone)]
pub enum InsertMasterState {
    Master,
    Confirm,
    Quit,
}

/// Represents the exit state of the insert master popup
///
/// # Variants
/// * `Confirm` - The confirm state
/// * `Quit` - The quit state
#[derive(Clone, PartialEq)]
pub enum InsertMasterExitState {
    Confirm,
    Quit,
}

/// Represents the insert master popup
///
/// # Fields
/// * `master` - The master
/// * `state` - The state
/// * `exit_state` - The exit state
/// * `x_percent` - The x percentage
/// * `y_percent` - The y percentage
///
/// # Methods
/// * `new` - Creates a new `InsertMaster`
///
/// # Implements
/// * `Popup` - The popup trait
#[derive(Clone)]
pub struct InsertMaster {
    pub master: String,
    pub state: InsertMasterState,
    pub exit_state: Option<InsertMasterExitState>,
    x_percent: u16,
    y_percent: u16,
}

impl InsertMaster {
    /// Creates a new insert master popup
    ///
    /// # Returns
    /// A new `InsertMaster`
    pub fn new() -> Self {
        InsertMaster {
            master: String::new(),
            state: InsertMasterState::Master,
            exit_state: None,
            x_percent: 40,
            y_percent: 20,
        }
    }

    /// Appends a character to the master
    ///
    /// # Arguments
    /// * `c` - The character to append
    pub fn master_append(&mut self, c: char) {
        self.master.push(c);
    }

    /// Pops a character from the master
    pub fn master_pop(&mut self) {
        self.master.pop();
    }
}

impl Popup for InsertMaster {
    fn render(&self, f: &mut Frame, _app: &Application, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Min(0)])
            .split(rect);

        let text = vec![Line::from(vec![Span::raw(self.master.clone())])];
        let master_p =
            Paragraph::new(text).block(Block::bordered().title("Master password").border_style(
                Style::default().fg(match self.state {
                    InsertMasterState::Master => Color::White,
                    _ => Color::DarkGray,
                }),
            ));

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(layout[1]);

        let quit_p = Paragraph::new(Span::raw("Quit")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertMasterState::Quit => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        let confirm_p = Paragraph::new(Span::raw("Confirm")).block(Block::bordered().border_style(
            Style::default().fg(match self.state {
                InsertMasterState::Confirm => Color::White,
                _ => Color::DarkGray,
            }),
        ));

        f.render_widget(Clear, rect);
        f.render_widget(master_p, layout[0]);
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
            InsertMasterState::Master => match key.code {
                KeyCode::Char(c) => {
                    self.master_append(c);
                }
                KeyCode::Backspace => {
                    self.master_pop();
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Enter | KeyCode::Up => {
                    self.state = InsertMasterState::Quit;
                }
                _ => {}
            },
            InsertMasterState::Quit => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertMasterExitState::Quit);
                    poped = true;
                }
                KeyCode::Up | KeyCode::Down => {
                    self.state = InsertMasterState::Master;
                }
                KeyCode::Right | KeyCode::Tab | KeyCode::Left => {
                    self.state = InsertMasterState::Confirm;
                }
                _ => {}
            },
            InsertMasterState::Confirm => match key.code {
                KeyCode::Enter => {
                    app.mutable_app_state.popups.pop();
                    self.exit_state = Some(InsertMasterExitState::Confirm);
                    poped = true;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.state = InsertMasterState::Quit;
                }
                KeyCode::Down | KeyCode::Tab | KeyCode::Up => {
                    self.state = InsertMasterState::Master;
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
        PopupType::InsertMaster
    }
}
