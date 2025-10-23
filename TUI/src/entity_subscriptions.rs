use futures::future::try_join;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};
use reqwest;
use std::sync::{Arc, RwLock};

use crate::{
    connected_apps::{ConnectedApp, query_connected_apps},
    entity_sharings::EntitySharing,
    shared::LoadingState,
};

#[derive(Clone, Debug, Default)]
pub struct EntitySubscriptionsWidget {
    state: Arc<RwLock<EntitySubscriptionsState>>,
}

#[derive(Debug, Default)]
struct EntitySubscriptionsState {
    loading_state: LoadingState,
    entity_subscriptions: Vec<EntitySubscription>,
    connected_apps: Vec<ConnectedApp>,
    list_state: ListState,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct EntitySubscription {
    id: String,
    connected_app_id: String,
    created_at: i64,
    updated_at: i64,
}

async fn query_entity_subscriptions(
    entity_sharing_id: &String,
) -> Result<Vec<EntitySubscription>, String> {
    let result = reqwest::get(format!(
        "http://localhost:8080/entity-sharings/{}/subscriptions",
        entity_sharing_id
    ))
    .await
    .map_err(|e| e.to_string())?;
    let body = result.text().await.map_err(|e| e.to_string())?;
    let entity_subscriptions: Vec<EntitySubscription> =
        serde_json::from_str(&body).map_err(|e| e.to_string())?;
    Ok(entity_subscriptions)
}

impl EntitySubscriptionsWidget {
    pub fn run(&self, entity_sharing: &EntitySharing) {
        let this = self.clone();
        let entity_sharing_clone = entity_sharing.clone();
        tokio::spawn(this.fetch_entity_subscriptions(entity_sharing_clone));
    }

    async fn fetch_entity_subscriptions(self, entity_sharing: EntitySharing) {
        self.state.write().unwrap().loading_state = LoadingState::Loading;

        let result = try_join(
            query_entity_subscriptions(&entity_sharing.id),
            query_connected_apps(),
        )
        .await;
        match result {
            Ok((entity_subscriptions, connected_apps)) => {
                let mut state = self.state.write().unwrap();
                state.entity_subscriptions = entity_subscriptions.to_vec();
                state.connected_apps = connected_apps.to_vec();
                state.loading_state = LoadingState::Loaded;
                if !&entity_subscriptions.is_empty() {
                    state.list_state.select(Some(0));
                }
            }
            Err(e) => {
                let mut state = self.state.write().unwrap();
                state.loading_state = LoadingState::Error(e.to_string());
            }
        };
    }

    pub fn scroll_down(&self) -> Option<EntitySubscription> {
        let mut state = self.state.write().unwrap();
        state.list_state.scroll_down_by(1);
        return Some(state.entity_subscriptions[state.list_state.selected().unwrap()].clone());
    }

    pub fn scroll_up(&self) -> Option<EntitySubscription> {
        let mut state = self.state.write().unwrap();
        state.list_state.scroll_up_by(1);
        return Some(state.entity_subscriptions[state.list_state.selected().unwrap()].clone());
    }
}

impl Widget for &EntitySubscriptionsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write().unwrap();

        let loading_state = Line::from(format!("{:?}", state.loading_state)).right_aligned();
        let block = Block::bordered()
            .title("Entity Subscriptions")
            .title(loading_state);

        let list = List::new(state.entity_subscriptions.iter().map(|subscription| {
            let connected_app = state
                .connected_apps
                .iter()
                .find(|app| app.id.eq(&subscription.connected_app_id));
            ListItem::from(format!(
                "Subcription from {}",
                connected_app.map(|app| app.name.clone()).unwrap_or("UNKNOWN APP".to_string())
            ))
        }))
        .block(block)
        .highlight_symbol(">> ");

        return StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
