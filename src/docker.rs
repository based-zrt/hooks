use std::env;

use actix_web::{post, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;

const ENV_KEY: &str = "DOCKER_BEARER";

#[post("/docker")]
async fn handle(auth: BearerAuth) -> impl Responder {
    let env_token = match env::var(ENV_KEY) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Missing environment value"})),
    };

    if env_token != auth.token() {
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::Accepted().finish()
}
