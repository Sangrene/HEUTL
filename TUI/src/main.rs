use color_eyre::{Report, Result};
use crossterm::event::{Event, EventStream, KeyCode};
use entity_sharings::{EntitySharing, EntitySharingsWidget};
use entity_subscriptions::EntitySubscriptionsWidget;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
};
use std::time::Duration;
use tokio;
use tokio_stream::StreamExt;

mod entity_sharings;
mod entity_subscriptions;
mod shared;
mod connected_apps;


#[derive(Debug)]
enum Error {
    EyreError(String),
}
impl From<Report> for Error {
    fn from(error: Report) -> Self {
        Error::EyreError(error.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
enum Screen {
    #[default]
    EntitySharingList,
    EntitySharingDetails,
    EntitySubscriptionDetails,
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    current_screen: Screen,
    entity_sharings: EntitySharingsWidget,
    entity_subscriptions: EntitySubscriptionsWidget,
    selected_entity_sharing: Option<EntitySharing>,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    async fn run(&mut self, mut terminal: ratatui::DefaultTerminal) -> Result<()> {
        self.entity_sharings.run();
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.render(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let layout =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(frame.area());
        let title = Line::from("HEUTL CLI").centered().bold();
        let body =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(layout[1]);

        frame.render_widget(title, layout[0]);
        frame.render_widget(&self.entity_sharings, body[0]);
        frame.render_widget(&self.entity_subscriptions, body[1]);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('j') | KeyCode::Down => {
                    self.selected_entity_sharing = self.entity_sharings.scroll_down();
                    if let Some(selected_entity_sharing) = self.selected_entity_sharing.clone() {
                        self.entity_subscriptions.run(&selected_entity_sharing);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected_entity_sharing = self.entity_sharings.scroll_up();
                    if let Some(selected_entity_sharing) = self.selected_entity_sharing.clone() {
                        self.entity_subscriptions.run(&selected_entity_sharing);
                    }
                }
                _ => {}
            }
        }
    }
}
