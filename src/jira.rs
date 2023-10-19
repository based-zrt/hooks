use std::env;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

const ENV_KEY: &str = "JIRA_TOKEN";

#[derive(Deserialize)]
struct Info {
    token: String,
}

#[post("/jira")]
async fn handle(info: web::Query<Info>) -> impl Responder {
    let env_token = match env::var(ENV_KEY) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Missing environment value"})),
    };

    if env_token != info.token {
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::Accepted().finish()
}
