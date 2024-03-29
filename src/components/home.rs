use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use futures::future::{abortable, Abortable};
use log::error;
use ratatui::{prelude::*, widgets::*};
use std::future::Future;
use std::{collections::HashMap, fs, thread, time::Duration};

use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use tracing::trace;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Component, Frame};
use crate::{action::Action, config::key_event_to_string};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Normal,
  Insert,
  Processing,
}

#[derive(Default)]
pub struct Home {
  pub show_help: bool,
  pub counter: usize,
  pub app_ticker: usize,
  pub render_ticker: usize,
  pub mode: Mode,
  pub input: Input,
  pub action_tx: Option<UnboundedSender<Action>>,
  pub keymap: HashMap<KeyEvent, Action>,
  pub text: Vec<String>,
  pub last_events: Vec<KeyEvent>,

  pub text_array: Vec<String>,
  pub text_current_word: String,
  pub text_current_index: usize,
  pub text_length: usize,
  pub text_play_on: bool,
  pub text_read_rate: u32,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
    self.keymap = keymap;
    self
  }

  pub fn tick(&mut self) {
    log::info!("Tick");
    self.app_ticker = self.app_ticker.saturating_add(1);
    self.last_events.drain(..);
  }

  pub fn render_tick(&mut self) {
    log::debug!("Render Tick");
    self.render_ticker = self.render_ticker.saturating_add(1);
  }

  pub fn add(&mut self, s: String) {
    self.text.push(s)
  }

  pub fn schedule_text_load(&mut self) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      // tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::TextLoad()).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn schedule_increment_text(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      // tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::IncrementText(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn schedule_decrement_text(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      // tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::DecrementText(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn schedule_sread_text(&mut self, i: usize) {
    self.text_play_on = !self.text_play_on;

    let mut handles = Vec::new();
    let j = self.text_length - self.text_current_index;
    let tx = self.action_tx.clone().unwrap();

    let text_current_index = self.text_current_index;
    let text_length = self.text_length;

    if self.text_play_on {
      handles.push(tokio::spawn(async move {
        while text_current_index < text_length {
          tokio::time::sleep(Duration::from_secs(1) / 2).await;
          tx.send(Action::EnterProcessing).unwrap();
          tx.send(Action::SreadText(i)).unwrap();
          tx.send(Action::ExitProcessing).unwrap();
        }
      }));
    } else {
      for handle in &handles {
        handle.abort();
        return;
      }
    }
  }

  // sreader
  pub fn text_load(&mut self) {
    let book: String =
      fs::read_to_string("./assets/lewisCarroll_alicesAdventuresInWonderland.txt").expect("failed to read file");
    self.text_array = book.split_whitespace().map(|s| s.to_string()).collect();
    self.text_current_word = self.text_array[0].clone();
    self.text_length = self.text_array.len();
  }
  pub fn sread_text(&mut self, i: usize) {
    if self.text_play_on {
      self.increment_text(i);
      if self.text_current_index == self.text_length - 1 {
        return;
      }
      return;
    }
  }
  pub fn increment_text(&mut self, i: usize) {
    if let Some(res) = self.text_current_index.checked_add(i) {
      self.text_current_index = res;
      self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
    }
  }
  pub fn decrement_text(&mut self, i: usize) {
    if let Some(res) = self.text_current_index.checked_sub(i) {
      self.text_current_index = res;
      self.text_current_word = self.text_array[self.text_current_index.clone()].clone();
    }
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.action_tx = Some(tx);
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    self.last_events.push(key.clone());
    let action = match self.mode {
      Mode::Normal | Mode::Processing => return Ok(None),
      Mode::Insert => match key.code {
        KeyCode::Esc => Action::EnterNormal,
        KeyCode::Enter => {
          if let Some(sender) = &self.action_tx {
            if let Err(e) = sender.send(Action::CompleteInput(self.input.value().to_string())) {
              error!("Failed to send action: {:?}", e);
            }
          }
          Action::EnterNormal
        },
        _ => {
          self.input.handle_event(&crossterm::event::Event::Key(key));
          Action::Update
        },
      },
    };
    Ok(Some(action))
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => self.tick(),
      Action::Render => self.render_tick(),
      Action::ToggleShowHelp => self.show_help = !self.show_help,
      Action::ScheduleIncrementText => self.schedule_increment_text(1),
      Action::ScheduleDecrementText => self.schedule_decrement_text(1),
      Action::ScheduleTextLoad => self.schedule_text_load(),
      Action::ScheduleSreadText => self.schedule_sread_text(1),
      Action::IncrementText(i) => self.increment_text(i),
      Action::DecrementText(i) => self.decrement_text(i),
      Action::TextLoad() => self.text_load(),
      Action::SreadText(i) => self.sread_text(i),
      Action::CompleteInput(s) => self.add(s),
      Action::EnterNormal => {
        self.mode = Mode::Normal;
      },
      Action::EnterInsert => {
        self.mode = Mode::Insert;
      },
      Action::EnterProcessing => {
        self.mode = Mode::Processing;
      },
      Action::ExitProcessing => {
        // TODO: Make this go to previous mode instead
        self.mode = Mode::Normal;
      },
      _ => (),
    }
    Ok(None)
  }

  // UI
  fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
    let rects = Layout::default().constraints([Constraint::Percentage(100), Constraint::Min(3)].as_ref()).split(rect);

    let mut text: Vec<Line> = self.text.clone().iter().map(|l| Line::from(l.clone())).collect();

    //    text.insert(0, "".into());
    //    text.insert(0, "Type into input and hit enter to display here".dim().into());
    //    text.insert(0, "".into());
    //    text.insert(0, format!("Render Ticker: {}", self.render_ticker).into());
    //    text.insert(0, format!("App Ticker: {}", self.app_ticker).into());
    //    text.insert(0, format!("Counter: {}", self.counter).into());

    text.insert(0, "".into());
    text.insert(0, "".into());
    text.insert(0, format!("Current Word: {}/{}", self.text_current_index, self.text_length).into());
    text.insert(0, "".into());
    text.insert(0, format!("{}", self.text_current_word).into());
    text.insert(0, "".into());
    text.insert(0, "".into());
    text.insert(
      0,
      Line::from(vec![
        "Press ".into(),
        Span::styled("j", Style::default().fg(Color::Red)),
        " or ".into(),
        Span::styled("k", Style::default().fg(Color::Red)),
        " to ".into(),
        Span::styled("increment", Style::default().fg(Color::Yellow)),
        " or ".into(),
        Span::styled("decrement", Style::default().fg(Color::Yellow)),
        ".".into(),
      ]),
    );
    text.insert(0, "".into());

    f.render_widget(
      Paragraph::new(text)
        .block(
          Block::default()
            .title("sreader")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(match self.mode {
              Mode::Processing => Style::default().fg(Color::Yellow),
              _ => Style::default(),
            })
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center),
      rects[0],
    );
    let width = rects[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = self.input.visual_scroll(width as usize);
    let input = Paragraph::new(self.input.value())
      .style(match self.mode {
        Mode::Insert => Style::default().fg(Color::Yellow),
        _ => Style::default(),
      })
      .scroll((0, scroll as u16))
      .block(Block::default().borders(Borders::ALL).title(Line::from(vec![
        Span::raw("Enter Input Mode "),
        Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
        Span::styled("/", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),
        Span::styled(" to start, ", Style::default().fg(Color::DarkGray)),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),
        Span::styled(" to finish)", Style::default().fg(Color::DarkGray)),
      ])));
    f.render_widget(input, rects[1]);
    if self.mode == Mode::Insert {
      f.set_cursor((rects[1].x + 1 + self.input.cursor() as u16).min(rects[1].x + rects[1].width - 2), rects[1].y + 1)
    }

    if self.show_help {
      let rect = rect.inner(&Margin { horizontal: 4, vertical: 2 });
      f.render_widget(Clear, rect);
      let block = Block::default()
        .title(Line::from(vec![Span::styled("Key Bindings", Style::default().add_modifier(Modifier::BOLD))]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
      f.render_widget(block, rect);
      let rows = vec![
        Row::new(vec!["?", "Open Help"]),
        Row::new(vec!["l", "Load Text"]),
        Row::new(vec!["Space", "Play/Pause Text"]),
        Row::new(vec!["j", "Increment Text"]),
        Row::new(vec!["k", "Decrement Text"]),
        Row::new(vec![""]),
        Row::new(vec!["/", "Enter Input"]),
        Row::new(vec!["ESC", "Exit Input"]),
        Row::new(vec!["Enter", "Submit Input"]),
        Row::new(vec!["q", "Quit"]),
      ];
      let table = Table::new(rows)
        .header(Row::new(vec!["Key", "Action"]).bottom_margin(1).style(Style::default().add_modifier(Modifier::BOLD)))
        .widths(&[Constraint::Percentage(10), Constraint::Percentage(90)])
        .column_spacing(1);
      f.render_widget(table, rect.inner(&Margin { vertical: 4, horizontal: 2 }));
    };

    f.render_widget(
      Block::default()
        .title(
          ratatui::widgets::block::Title::from(format!(
            "{:?}",
            &self.last_events.iter().map(|k| key_event_to_string(k)).collect::<Vec<_>>()
          ))
          .alignment(Alignment::Right),
        )
        .title_style(Style::default().add_modifier(Modifier::BOLD)),
      Rect { x: rect.x + 1, y: rect.height.saturating_sub(1), width: rect.width.saturating_sub(2), height: 1 },
    );

    Ok(())
  }
}
