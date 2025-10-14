use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::entity_sharing::entity_sharing_core::EntitySharingCore;
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use axum::{extract::{State, Path}, response::IntoResponse, routing::{get, post}, Json, Router, debug_handler};
use reqwest::StatusCode;
use tokio::net::TcpListener;
use std::sync::{Arc};
use serde_json::Value;
use std::io::Error;
use tokio::signal;


#[debug_handler]
async fn get_connected_apps(State(web_app_cores): State<WebAppCores>) -> impl IntoResponse {
    let connected_apps = web_app_cores
        .app_core
        .get_all_connected_apps()
        .await
        .unwrap();
    return (StatusCode::OK, Json(connected_apps));
}

#[debug_handler]
async fn get_entity_sharings(State(web_app_cores): State<WebAppCores>) -> impl IntoResponse {
    let entity_sharings = web_app_cores
        .entity_sharing_core
        .get_all_polling_entity_sharings()
        .await
        .unwrap();
    return (StatusCode::OK, Json(entity_sharings));
}

#[debug_handler]
async fn notify_new_entity_list(
    State(web_app_cores): State<WebAppCores>,
    Path(entity_sharing_id): Path<String>,
    Json(data): Json<Value>,
) -> impl IntoResponse {
    let result = web_app_cores
        .entity_subscription_core
        .notify_all_subscriptions_of_new_entity_list(&entity_sharing_id, &data)
        .await
        .unwrap();

    return (StatusCode::OK, Json("ok"));
}

#[derive(Clone)]
struct WebAppCores {
    pub app_core: Arc<ConnectedAppCore<'static>>,
    pub entity_sharing_core: Arc<EntitySharingCore<'static>>,
    pub entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub async fn run_web_api(
    app_core: Arc<ConnectedAppCore<'static>>,
    entity_sharing_core: Arc<EntitySharingCore<'static>>,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
) -> Result<(), Error> {
    let app = Router::new()
    .route("/connected-apps", get(get_connected_apps))
    .route("/entity-sharings", get(get_entity_sharings))
    .route("/entity/{entity_sharing_id}", post(notify_new_entity_list))
    .with_state(WebAppCores {
        app_core,
        entity_sharing_core,
        entity_subscription_core,
    });

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;
    Ok(())
}
