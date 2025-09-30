use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::connected_app::connected_app_repository::connected_app_sqlite_repository::ConnectedAppSQLiteRepository;
use crate::entity_sharing::entity_sharing_core::{ EntitySharingCore};
use crate::entity_sharing::entity_sharing_model::EntitySharingPollingInfos;
use crate::entity_sharing::entity_sharing_repository::{CreateEntitySharingParams};
use crate::entity_sharing::entity_sharing_repository::entity_sharing_sqlite_repository::EntitySharingSQLiteRepository;
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use crate::entity_subscription::entity_subscription_repository::entity_subscription_sqlite_repository::EntitySubscriptionSQLiteRepository;
use crate::entity_subscription::entity_subscription_repository::CreateEntitySubscriptionParams;
use crate::connected_app::connected_app_repository::CreateConnectedAppParams;
use crate::shared::db::get_db;
use crate::entity_sharing::entity_polling_handler::EntityPollingHandler;
use crate::shared::bus::{Commands, TopicIds};
use pubsub_bus::{EventBus, EventEmitter};
use serde_json::json;
use uuid::{Uuid, Timestamp, NoContext};
use std::collections::HashMap;
use chrono::{Timelike, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio;

mod connected_app;
mod entity_sharing;
mod entity_subscription;
mod shared;

async fn test_scenario<'a>(
    app_core: Arc<ConnectedAppCore<'a>>,
    entity_sharing_core: Arc<Mutex<EntitySharingCore<'a>>>,
    entity_subscription_core: Arc<EntitySubscriptionCore<'a>>,
) {
    let ts = Timestamp::from_unix(
        NoContext,
        (Utc::now().timestamp() * 1000).try_into().unwrap(),
        0,
    );
    let aptimize_app = app_core
        .create_connected_app(&CreateConnectedAppParams {
            id: Uuid::new_v7(ts).to_string(),
            name: "Aptimize".to_string(),
        })
        .await
        .unwrap();
    let arcfm_app = app_core
        .create_connected_app(&CreateConnectedAppParams {
            id: Uuid::new_v7(ts).to_string(),
            name: "ArcFM".to_string(),
        })
        .await
        .unwrap();
    let aptimize_asset = entity_sharing_core
        .lock()
        .unwrap()
        .create_entity_sharing_with_polling(&CreateEntitySharingParams {
            id: Uuid::new_v7(ts).to_string(),
            name: "asset".to_string(),
            connected_app_id: aptimize_app.id.clone(),
            json_schema: json!({}),
            polling_infos: Some(EntitySharingPollingInfos {
                polling_interval: 1,
                polling_url: "https://api.app.aptimize.com".to_string(),
                polling_method: "GET".to_string(),
                polling_headers: HashMap::new(),
                polling_body: "".to_string(),
                polling_timeout: 1,
                polling_retries: 1,
                polling_retry_delay: 1,
            }),
        })
        .await
        .unwrap();

    let arcfm_asset = entity_sharing_core
        .lock()
        .unwrap()
        .create_entity_sharing_with_polling(&CreateEntitySharingParams {
            id: Uuid::new_v7(ts).to_string(),
            name: "asset".to_string(),
            connected_app_id: arcfm_app.id.clone(),
            json_schema: json!({}),
            polling_infos: Some(EntitySharingPollingInfos {
                polling_interval: 1,
                polling_url: "https://arcfm.com".to_string(),
                polling_method: "GET".to_string(),
                polling_headers: HashMap::new(),
                polling_body: "".to_string(),
                polling_timeout: 1,
                polling_retries: 1,
                polling_retry_delay: 1,
            }),
        })
        .await
        .unwrap();

    let aptimize_subscription = entity_subscription_core
        .create_entity_subscription(&CreateEntitySubscriptionParams {
            id: Uuid::new_v7(ts).to_string(),
            entity_sharing_id: arcfm_asset.id.clone(),
            connected_app_id: aptimize_app.id.clone(),
            jdm_transform: None,
        })
        .await
        .unwrap();
    let arcfm_subscription = entity_subscription_core
        .create_entity_subscription(&CreateEntitySubscriptionParams {
            id: Uuid::new_v7(ts).to_string(),
            entity_sharing_id: aptimize_asset.id.clone(),
            connected_app_id: arcfm_app.id.clone(),
            jdm_transform: None,
        })
        .await
        .unwrap();
}

async fn run_app() {
    let bus: EventBus<Commands, TopicIds> = EventBus::new();

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = Arc::clone(&should_stop);
    ctrlc::set_handler(move || {
        println!("Received SIGINT/SIGTERM, shutting down...");
        should_stop_clone.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");
    let pool = Box::leak(Box::new(get_db().await.expect("Failed to create database")));

    let connected_app_repository = Box::new(ConnectedAppSQLiteRepository { pool: pool });
    let entity_sharing_repository = Box::new(EntitySharingSQLiteRepository { pool: pool });
    let entity_subscription_repository =
        Box::new(EntitySubscriptionSQLiteRepository { pool: pool });

    let app_core = Arc::new(ConnectedAppCore {
        connected_app_repository: connected_app_repository,
    });
    let entity_sharing_core = Arc::new(Mutex::new(EntitySharingCore::new(
        EventEmitter::new(),
        Arc::clone(&app_core),
        entity_sharing_repository,
    )));
 
    bus.add_publisher(&mut *entity_sharing_core.lock().unwrap(), None)
        .expect("Failed to add publisher");

    let entity_subscription_core = Arc::new(EntitySubscriptionCore {
        entity_subscription_repository: entity_subscription_repository,
        entity_sharing_core: Arc::clone(&entity_sharing_core),
    });

    let entity_polling_handler = EntityPollingHandler::new(
        Arc::clone(&entity_subscription_core),
        Arc::clone(&entity_sharing_core),
        Arc::clone(&should_stop),
    );

    bus.add_subscriber(entity_polling_handler);

    test_scenario(
        Arc::clone(&app_core),
        Arc::clone(&entity_sharing_core),
        Arc::clone(&entity_subscription_core),
    )
    .await;

    while !should_stop.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() {
    run_app().await;
}
