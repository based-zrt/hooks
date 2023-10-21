use std::env;
use std::sync::Arc;

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;

use crate::Clients;

const ENV_KEY: &str = "JIRA_TOKEN";

#[derive(Deserialize)]
struct Info {
    token: String,
}

#[post("/jira")]
async fn handle(info: web::Query<Info>, req_body: String, clients: Data<Arc<Clients>>) -> impl Responder {
    let env_token = match env::var(ENV_KEY) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Missing environment value"})),
    };

    if env_token != info.token {
        return HttpResponse::Unauthorized().finish();
    }

    let _ = clients
        .jira_client
        .send(|message| message.username("Jira").content(format!("```{}```", req_body).as_str()))
        .await;

    HttpResponse::Accepted().finish()
}
