use color_eyre::Result;
use ratatui::{
  layout::Rect,
  style::{Color, Style, Stylize},
  widgets::{Block, Borders},
  Frame,
};
use tui_textarea::{Input, TextArea};

use super::Component;
use crate::action::Action;

pub struct Repl {
  textarea: TextArea<'static>,
}

impl Repl {
  pub fn new() -> Self {
    Self {
      textarea: TextArea::default(),
    }
  }
}

impl Component for Repl {
  fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
    match key.code {
      crossterm::event::KeyCode::Tab => return Ok(Some(Action::SwitchFocusForward)),
      crossterm::event::KeyCode::BackTab => return Ok(Some(Action::SwitchFocusBackward)),
      _ => {}
    }

    self.textarea.input(Input::from(key));
    Ok(None)
  }

  fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool) -> Result<()> {
    let block = Block::default()
      .borders(Borders::ALL)
      .title("REPL")
      .border_style(
        focused
          .then(|| Style::default().fg(Color::LightGreen))
          .unwrap_or_default(),
      );
    self.textarea.set_block(block);
    self.textarea.set_cursor_style(
      focused
        .then(|| Style::default().bg(Color::White))
        .unwrap_or(self.textarea.cursor_line_style()),
    );
    self.textarea.set_cursor_line_style(
      focused
        .then(|| Style::default().fg(Color::White))
        .unwrap_or(Style::default()),
    );
    frame.render_widget(&self.textarea, area);
    Ok(())
  }

  fn update(&mut self, _action: Action) -> Result<Option<Action>> {
    Ok(None)
  }
}
