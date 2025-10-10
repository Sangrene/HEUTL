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
use crate::services::web_api::run_web_api;
use crate::shared::db::get_db;
use crate::entity_sharing::entity_polling_handler::EntityPollingHandler;
use crate::shared::bus::{Commands, TopicIds};
use pubsub_bus::{EventBus, EventEmitter};
use serde_json::{json, Value};
use uuid::{Uuid, Timestamp, NoContext};
use chrono::{Timelike, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

mod connected_app;
mod entity_sharing;
mod entity_subscription;
mod services;
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
            name: "Aptimize asset".to_string(),
            connected_app_id: aptimize_app.id.clone(),
            json_schema: json!({}),
            python_script: Some("result = [{\"name\": \"aptimize_asset1\"}]".to_string()),
            data_path: Some("data".to_string()),
            is_array: true,
            polling_infos: Some(EntitySharingPollingInfos {
                polling_interval: 10000,
            }),
        })
        .await
        .unwrap();

    let arcfm_asset = entity_sharing_core
        .lock()
        .unwrap()
        .create_entity_sharing_with_polling(&CreateEntitySharingParams {
            id: Uuid::new_v7(ts).to_string(),
            name: "ArcFM asset".to_string(),
            connected_app_id: arcfm_app.id.clone(),
            json_schema: json!({}),
            python_script: Some("result = [{\"name\": \"arcfm_asset1\"}]".to_string()),
            data_path: Some("assets".to_string()),
            is_array: true,
            polling_infos: Some(EntitySharingPollingInfos {
                polling_interval: 1000,
            }),
        })
        .await
        .unwrap();

    entity_subscription_core
        .create_entity_subscription(&CreateEntitySubscriptionParams {
            id: Uuid::new_v7(ts).to_string(),
            entity_sharing_id: arcfm_asset.id.clone(),
            connected_app_id: aptimize_app.id.clone(),
            jdm_transform: None,
            python_script: None,
        })
        .await
        .unwrap();
    entity_subscription_core
        .create_entity_subscription(&CreateEntitySubscriptionParams {
            id: Uuid::new_v7(ts).to_string(),
            entity_sharing_id: aptimize_asset.id.clone(),
            connected_app_id: arcfm_app.id.clone(),
            jdm_transform: None,
            python_script: None,
        })
        .await
        .unwrap();
}

async fn init_app() -> (
    Arc<ConnectedAppCore<'static>>,
    Arc<Mutex<EntitySharingCore<'static>>>,
    Arc<EntitySubscriptionCore<'static>>,
    Arc<AtomicBool>,
) {
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

    (
        app_core,
        entity_sharing_core,
        entity_subscription_core,
        should_stop,
    )
}

async fn run_app() {
    let (app_core, entity_sharing_core, entity_subscription_core, should_stop) = init_app().await;

    test_scenario(
        Arc::clone(&app_core),
        Arc::clone(&entity_sharing_core),
        Arc::clone(&entity_subscription_core),
    )
    .await;

    run_web_api(app_core, entity_sharing_core, entity_subscription_core)
        .await
        .expect("Failed to run web api");
}

#[actix_web::main]
async fn main() {
    run_app().await;
}
