use axum::{Json, debug_handler, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use crate::services::web_api::WebAppCores;
use crate::connected_app::connected_app_repository::CreateConnectedAppParams;

#[debug_handler]
pub async fn get_connected_apps(State(web_app_cores): State<WebAppCores>) -> impl IntoResponse {
    let connected_apps = web_app_cores
        .app_core
        .get_all_connected_apps()
        .await
        .unwrap();
    return (StatusCode::OK, Json(connected_apps));
}

#[debug_handler]
pub async fn create_connected_app(
    State(web_app_cores): State<WebAppCores>,
    Json(data): Json<CreateConnectedAppParams>,
) -> impl IntoResponse {
    let connected_app = web_app_cores
        .app_core
        .create_connected_app(&data)
        .await
        .unwrap();
    return (StatusCode::CREATED, Json(connected_app));
}
