use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Style},
  widgets::{Block, BorderType, Borders, Paragraph},
};
use tracing::debug;

use super::{logger::Logger, Component, Frame};
use crate::action::Action;

#[derive(Default)]
pub struct Home {
  pub is_running: bool,
  pub show_logger: bool,
  pub logger: Logger,
  pub counter: usize,
}

impl Home {
  pub fn tick(&self) {
    debug!("Tick");
  }

  pub fn increment(&mut self, i: usize) {
    self.counter = self.counter.saturating_add(i);
  }

  pub fn decrement(&mut self, i: usize) {
    self.counter = self.counter.saturating_sub(i);
  }
}

impl Component for Home {
  fn init(&mut self) -> anyhow::Result<()> {
    self.is_running = true;
    Ok(())
  }

  fn on_key_event(&self, key: KeyEvent) -> Action {
    match key.code {
      KeyCode::Char('q') => Action::Quit,
      KeyCode::Char('l') => Action::ToggleShowLogger,
      KeyCode::Char('j') => Action::IncrementCounter,
      KeyCode::Char('k') => Action::DecrementCounter,
      _ => Action::Tick,
    }
  }

  fn dispatch(&mut self, action: Action) -> Option<Action> {
    match action {
      Action::Quit => self.is_running = false,
      Action::Tick => self.tick(),
      Action::ToggleShowLogger => self.show_logger = !self.show_logger,
      Action::IncrementCounter => self.increment(1),
      Action::DecrementCounter => self.decrement(1),
      _ => (),
    }
    None
  }

  fn render(&mut self, f: &mut Frame<'_>, rect: Rect) {
    let rect = if self.show_logger {
      let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);
      self.logger.render(f, chunks[1]);
      chunks[0]
    } else {
      rect
    };

    f.render_widget(
      Paragraph::new(format!("Press j or k to increment or decrement.\n\nCounter: {}", self.counter))
        .block(
          Block::default()
            .title("Template")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center),
      rect,
    )
  }
}
