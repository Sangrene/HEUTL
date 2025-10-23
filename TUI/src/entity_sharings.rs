use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};
use reqwest;
use std::sync::{Arc, RwLock};

use crate::shared::LoadingState;

#[derive(Clone, Debug, Default)]
pub struct EntitySharingsWidget {
    state: Arc<RwLock<EntitySharingsState>>,
}

#[derive(Debug, Default)]
struct EntitySharingsState {

    loading_state: LoadingState,
    entity_sharings: Vec<EntitySharing>,
    list_state: ListState,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct EntitySharing {
    name: String,
    pub id: String,
    created_at: i64,
    updated_at: i64,
}

async fn query_entity_sharings() -> Result<Vec<EntitySharing>, String> {
    let result = reqwest::get("http://localhost:8080/entity-sharings")
        .await
        .map_err(|e| e.to_string())?;
    let body = result.text().await.map_err(|e| e.to_string())?;
    let entity_sharings: Vec<EntitySharing> =
        serde_json::from_str(&body).map_err(|e| e.to_string())?;
    Ok(entity_sharings)
}

impl EntitySharingsWidget {
    pub fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_entity_sharings());
    }

    async fn fetch_entity_sharings(self) {
        self.state.write().unwrap().loading_state = LoadingState::Loading;
        let entity_sharings = query_entity_sharings().await;
        match entity_sharings {
            Ok(entity_sharings) => {
                let mut state = self.state.write().unwrap();
                state.entity_sharings = entity_sharings.to_vec();
                state.loading_state = LoadingState::Loaded;
                if !&entity_sharings.is_empty() {
                    state.list_state.select(Some(0));
                }
            }
            Err(e) => {
                let mut state = self.state.write().unwrap();
                state.loading_state = LoadingState::Error(e);
            }
        }
    }

    pub fn scroll_down(&self) -> Option<EntitySharing> {
        let mut state = self.state.write().unwrap();
        state.list_state.scroll_down_by(1);
        return Some(state.entity_sharings[state.list_state.selected().unwrap()].clone());
    }

    pub fn scroll_up(&self) -> Option<EntitySharing> {
        let mut state = self.state.write().unwrap();
        state.list_state.scroll_up_by(1);
        return Some(state.entity_sharings[state.list_state.selected().unwrap()].clone());
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

        let list = List::new(
            state
                .entity_sharings
                .iter()
                .map(|sharing| ListItem::from(format!("{}", sharing.name))),
        )
        .block(block)
        .highlight_symbol(">> ");

        return StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
