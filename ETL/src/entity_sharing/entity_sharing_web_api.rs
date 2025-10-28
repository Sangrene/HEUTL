use crate::entity_sharing::entity_sharing_repository::{
    CreateEntitySharingParams, UpdateEntitySharingParams,
};
use crate::services::web_api::WebAppCores;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde_json::Value;

#[debug_handler]
pub async fn create_entity_sharing(
    State(web_app_cores): State<WebAppCores>,
    Json(data): Json<CreateEntitySharingParams>,
) -> impl IntoResponse {
    let entity_sharing = web_app_cores
        .entity_sharing_core
        .create_entity_sharing(&data)
        .await
        .unwrap();
    return (StatusCode::CREATED, Json(entity_sharing));
}

#[debug_handler]
pub async fn notify_new_entity_list(
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

#[debug_handler]
pub async fn get_entity_sharings(State(web_app_cores): State<WebAppCores>) -> impl IntoResponse {
    let entity_sharings = web_app_cores
        .entity_sharing_core
        .get_all_polling_entity_sharings()
        .await
        .unwrap();
    return (StatusCode::OK, Json(entity_sharings));
}

#[debug_handler]
pub async fn update_entity_sharing(
    State(web_app_cores): State<WebAppCores>,
    Path(entity_sharing_id): Path<String>,
    Json(data): Json<UpdateEntitySharingParams>,
) -> impl IntoResponse {
    let entity_sharing = web_app_cores
        .entity_sharing_core
        .update_entity_sharing(&entity_sharing_id, &data)
        .await
        .unwrap();
    return (StatusCode::OK, Json(entity_sharing));
}
