use ratatui::{text::Line, widgets::{List, ListItem, ListState, StatefulWidget, Widget}};
use reqwest;
use std::sync::{Arc, RwLock};

use crate::shared::LoadingState;

struct EntitySharingsWidget {
    state: Arc<RwLock<EntitySharingsState>>,
}

struct EntitySharingsState {
    loading_state: LoadingState,
    entity_sharings: Vec<EntitySharing>,
    list_state: ListState,
}

struct EntitySharing {
    name: String,
    id: String,
    created_at: String,
    updated_at: String,
}

impl EntitySharingsWidget {
    fn run(&self) {
        let this = self.clone(); // clone the widget to pass to the background task
        tokio::spawn(this.fetch_entity_sharings());
    }

    async fn fetch_entity_sharings(self) {
        self.state.write().unwrap().loading_state = LoadingState::Loading;
        match {
            let result = reqwest::get("http://localhost:8000/entity_sharings")
                .await
                .map_err(|e| LoadingState::Error(e.to_string()))
                .m;
            let body = result.text().await?;
            let entity_sharings: Vec<EntitySharing> = serde_json::from_str(&body)?;
            return entity_sharings;
        } {
            Ok(entity_sharings) => {
                self.state.write().unwrap().entity_sharings = entity_sharings;
                self.state.write().unwrap().loading_state = LoadingState::Loaded;
            }
            Err(e) => {
                self.state.write().unwrap().loading_state = LoadingState::Error(e.to_string());
            }
        }
    }
}

impl Widget for &EntitySharingsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write().unwrap();
        let loading_state = Line::from(format!("{:?}", state.loading_state)).right_aligned();
        let block = Block::bordered()
            .title("Entity Sharings")
            .title(loading_state)
            .title_bottom("↑/↓ to scroll, q to quit");

        let list = List::new(state.entity_sharings.iter().map(|sharing| ListItem::from(Line::styled(format!(""), style))))

        return StatefulWidget::render(self, area, buf, &mut state.list_state);
    }
}