use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
  layout::Rect,
  style::{Color, Modifier, Style},
  widgets::{Block, Borders},
  Frame,
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use super::Component;
use crate::action::Action;

pub struct KeyspaceList {
  keyspaces: Vec<TreeItem<'static, String>>,
  state: TreeState<String>,
}

impl KeyspaceList {
  pub fn new() -> Self {
    let keyspaces = vec![
      TreeItem::new_leaf("keyspace1".to_string(), "keyspace1".to_string()),
      TreeItem::new(
        "keyspace2".to_string(),
        "keyspace2".to_string(),
        vec![
          TreeItem::new_leaf("table1".to_string(), "table1".to_string()),
          TreeItem::new_leaf("table2".to_string(), "table2".to_string()),
        ],
      )
      .expect("Tree to work"),
    ];
    Self {
      keyspaces,
      state: TreeState::default(),
    }
  }
}

impl Component for KeyspaceList {
  fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool) -> Result<()> {
    let tree = Tree::new(&self.keyspaces)?
      .block(
        Block::default()
          .borders(Borders::ALL)
          .title("Keyspaces")
          .border_style(
            focused
              .then(|| Style::default().fg(Color::LightGreen))
              .unwrap_or_default(),
          ),
      )
      .highlight_style(
        Style::default()
          .fg(Color::Black)
          .bg(Color::LightGreen)
          .add_modifier(Modifier::BOLD),
      );
    frame.render_stateful_widget(tree, area, &mut self.state);
    Ok(())
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    match key.code {
      KeyCode::Tab => return Ok(Some(Action::SwitchFocusForward)),
      KeyCode::BackTab => return Ok(Some(Action::SwitchFocusBackward)),
      KeyCode::Char('\n') | KeyCode::Char(' ') => {
        self.state.toggle_selected();
      }
      KeyCode::Left | KeyCode::Char('H') | KeyCode::Char('h') => {
        self.state.key_left();
      }
      KeyCode::Right | KeyCode::Char('L') | KeyCode::Char('l') => {
        self.state.key_right();
      }
      KeyCode::Down | KeyCode::Char('J') | KeyCode::Char('j') => {
        self.state.key_down();
      }
      KeyCode::Up | KeyCode::Char('K') | KeyCode::Char('k') => {
        self.state.key_up();
      }
      KeyCode::Esc => {
        self.state.select(Vec::new());
      }
      KeyCode::Home => {
        self.state.select_first();
      }
      KeyCode::End => {
        self.state.select_last();
      }
      KeyCode::PageDown => {
        self.state.scroll_down(3);
      }
      KeyCode::PageUp => {
        self.state.scroll_up(3);
      }
      _ => {}
    }
    Ok(None)
  }

  fn update(&mut self, _action: Action) -> Result<Option<Action>> {
    Ok(None)
  }
}
