use std::env;
use std::sync::Arc;

use actix_web::{post, web::Data, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;

use crate::Clients;

const ENV_KEY: &str = "DOCKER_BEARER";

#[post("/docker")]
async fn handle(auth: BearerAuth, req_body: String, clients: Data<Arc<Clients>>) -> impl Responder {
    let env_token = match env::var(ENV_KEY) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Missing environment value"})),
    };

    if env_token != auth.token() {
        return HttpResponse::Unauthorized().finish();
    }

    let _ = clients
        .docker_client
        .send(|message| {
            message
                .username("Docker")
                .content(format!("```{}```", req_body).as_str())
        })
        .await;

    HttpResponse::Accepted().finish()
}
