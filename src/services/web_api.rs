use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::entity_sharing::entity_sharing_core::EntitySharingCore;
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/connected-apps")]
async fn get_connected_apps(web_app_cores: web::Data<WebAppCores>) -> impl Responder {
    let connected_apps = web_app_cores
        .app_core
        .get_all_connected_apps()
        .await
        .unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&connected_apps).unwrap())
}

#[get("/entity-sharings")]
async fn get_entity_sharings(web_app_cores: web::Data<WebAppCores>) -> impl Responder {
    let entity_sharings = web_app_cores
        .entity_sharing_core
        .lock()
        .unwrap()
        .get_all_polling_entity_sharings()
        .await
        .unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&entity_sharings).unwrap())
}

#[post("/entity/{entity_sharing_id}")]
async fn notify_new_entity_list(
    web_app_cores: web::Data<WebAppCores>,
    entity_sharing_id: web::Path<String>,
    body: String
) -> impl Responder {
    let entity_sharing_id = entity_sharing_id.to_string();
    let data = serde_json::from_str::<Value>(&body).unwrap();
    let result = web_app_cores
        .entity_subscription_core
        .notify_all_subscriptions_of_new_entity_list(&entity_sharing_id, &data)
        .await
        .unwrap();

    return HttpResponse::Ok().body("ok");
}

struct WebAppCores {
    pub app_core: Arc<ConnectedAppCore<'static>>,
    pub entity_sharing_core: Arc<Mutex<EntitySharingCore<'static>>>,
    pub entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
}

pub async fn run_web_api(
    app_core: Arc<ConnectedAppCore<'static>>,
    entity_sharing_core: Arc<Mutex<EntitySharingCore<'static>>>,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(WebAppCores {
                app_core: app_core.clone(),
                entity_sharing_core: entity_sharing_core.clone(),
                entity_subscription_core: entity_subscription_core.clone(),
            }))
            .service(hello)
            .service(get_connected_apps)
            .service(get_entity_sharings)
            .service(notify_new_entity_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
