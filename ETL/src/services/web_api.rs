use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::connected_app::connected_app_web_api::{create_connected_app, get_connected_apps};
use crate::entity_sharing::entity_sharing_core::EntitySharingCore;
use crate::entity_sharing::entity_sharing_web_api::{
    create_entity_sharing, get_entity_sharings, notify_new_entity_list, update_entity_sharing,  
};
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use crate::entity_subscription::entity_subscription_web_api::{
    create_entity_subscription, get_entity_subscriptions,
};
use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post, put},
};
use std::io::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("Request {:?}::{:?}", req.method(), req.uri());

    let response = next.run(req).await;
    return response;
}

#[derive(Clone)]
pub struct WebAppCores {
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
        .route(
            "/entity-sharings/{entity_sharing_id}/subscriptions",
            get(get_entity_subscriptions),
        )
        .route("/entity/{entity_sharing_id}", post(notify_new_entity_list))
        .route("/entity-sharings", post(create_entity_sharing))
        .route("/entity-sharings/{entity_sharing_id}", put(update_entity_sharing))
        .route("/entity-subscriptions", post(create_entity_subscription))
        .route("/connected-apps", post(create_connected_app))
        .layer(middleware::from_fn(logging_middleware))
        .with_state(WebAppCores {
            app_core,
            entity_sharing_core,
            entity_subscription_core,
        });

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
