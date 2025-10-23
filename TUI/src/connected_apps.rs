#[derive(Debug, serde::Deserialize, Clone)]
pub struct ConnectedApp {
    pub id: String,
    pub name: String,
    created_at: i64,
    updated_at: i64,
}

pub async fn query_connected_apps() -> Result<Vec<ConnectedApp>, String> {
    let result = reqwest::get("http://localhost:8080/connected-apps")
        .await
        .map_err(|e| e.to_string())?;
    let body = result.text().await.map_err(|e| e.to_string())?;
    let connected_apps: Vec<ConnectedApp> =
        serde_json::from_str(&body).map_err(|e| e.to_string())?;
    Ok(connected_apps)
}