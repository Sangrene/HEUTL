use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::connected_app::connected_app_repository::connected_app_sqlite_repository::ConnectedAppSQLiteRepository;
use crate::entity_sharing::entity_sharing_core::{ EntitySharingCore};
use crate::entity_sharing::entity_sharing_repository::{CreateEntitySharingParams};
use crate::entity_sharing::entity_sharing_repository::entity_sharing_sqlite_repository::EntitySharingSQLiteRepository;
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use crate::entity_subscription::entity_subscription_repository::entity_subscription_sqlite_repository::EntitySubscriptionSQLiteRepository;
use crate::entity_subscription::entity_subscription_repository::CreateEntitySubscriptionParams;
use crate::connected_app::connected_app_repository::CreateConnectedAppParams;
use crate::shared::db::DB;
use serde_json::json;

mod connected_app;
mod entity_sharing;
mod entity_subscription;
mod shared;
mod subscription_manager;

async fn test_scenario<'a>(
    app_core: &ConnectedAppCore<'a>,
    entity_sharing_core: &EntitySharingCore<'a>,
    entity_subscription_core: &EntitySubscriptionCore<'a>,
) {
    let app = app_core
        .create_connected_app(&CreateConnectedAppParams {
            id: "test".to_string(),
            name: "test".to_string(),
        })
        .await
        .unwrap();
    let entity_sharing = entity_sharing_core
        .create_entity_sharing_with_polling(&CreateEntitySharingParams {
            id: "test".to_string(),
            name: "test".to_string(),
            connected_app_id: app.id.clone(),
            jdm_transform: None,
            json_schema: json!({}),
            polling_infos: None,
        })
        .await
        .unwrap();
    let entity_subscription = entity_subscription_core
        .create_entity_subscription(&CreateEntitySubscriptionParams {
            id: "test".to_string(),
            entity_sharing_id: entity_sharing.id.clone(),
            connected_app_id: app.id.clone(),
        })
        .await
        .unwrap();
    println!("App: {:?}", app);
    println!("Entity Sharing: {:?}", entity_sharing);
    println!("Entity Subscription: {:?}", entity_subscription);
}

async fn run_app() {
    let db = DB::new().await.expect("Failed to create database");
    let pool = db.get_pool();
    
    let app_core = ConnectedAppCore {
        connected_app_repository: Box::new(ConnectedAppSQLiteRepository { pool: &pool }),
    };
    let entity_sharing_core = EntitySharingCore {
        connected_app_core: &app_core,
        entity_sharing_repository: Box::new(EntitySharingSQLiteRepository { pool: &pool }),
    };

    let entity_subscription_core = EntitySubscriptionCore {
        entity_subscription_repository: Box::new(EntitySubscriptionSQLiteRepository {
            pool: &pool,
        }),
        entity_sharing_core: &entity_sharing_core,
    };
    test_scenario(&app_core, &entity_sharing_core, &entity_subscription_core).await;
}

#[tokio::main]
async fn main() {
    run_app().await;
}
