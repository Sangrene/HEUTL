use crate::entity_subscription::entity_subscription_repository::CreateEntitySubscriptionParams;
use crate::services::web_api::WebAppCores;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;


#[debug_handler]
pub async fn get_entity_subscriptions(
    State(web_app_cores): State<WebAppCores>,
    Path(entity_sharing_id): Path<String>,
) -> impl IntoResponse {
    let entity_subscriptions = web_app_cores
        .entity_subscription_core
        .get_all_entity_subscriptions_for_entity_sharing(&entity_sharing_id)
        .await
        .unwrap();
    return (StatusCode::OK, Json(entity_subscriptions));
}

#[debug_handler]
pub async fn create_entity_subscription(
    State(web_app_cores): State<WebAppCores>,
    Json(data): Json<CreateEntitySubscriptionParams>,
) -> impl IntoResponse {
    let entity_subscription = web_app_cores
        .entity_subscription_core
        .create_entity_subscription(&data)
        .await
        .unwrap();
    return (StatusCode::CREATED, Json(entity_subscription));
}
