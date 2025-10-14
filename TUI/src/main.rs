use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tokio;
mod entity_sharings;
mod shared;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default=Screen::ConnectedApps)]
enum Screen {
    ConnectedApps,
    EntitySharingList,
    EntitySharingDetails,
    EntitySubscriptionDetails
  }

  
#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    current_screen: Screen,
}