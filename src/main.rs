mod docker;
mod jira;
mod types;

use std::env;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::extractors::bearer;
use dotenv::dotenv;
use webhook::client::WebhookClient;

struct Clients {
    jira_client: WebhookClient,
    docker_client: WebhookClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let jira_url = env::var("JIRA_URL").expect("missing jira url");

    let docker_url = env::var("DOCKER_URL").expect("missing docker url");

    let data = Arc::new(Clients {
        jira_client: WebhookClient::new(&jira_url),
        docker_client: WebhookClient::new(&docker_url),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(bearer::Config::default())
            .app_data(Data::new(data.clone()))
            .service(jira::handle)
            .service(docker::handle)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
