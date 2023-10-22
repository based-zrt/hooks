mod docker;
mod jira;

use std::sync::Arc;
use std::{env, process};

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::extractors::bearer;
use webhook::client::WebhookClient;

struct Clients {
    jira_client: WebhookClient,
    docker_client: WebhookClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let jira_url = match env::var("JIRA_URL") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    let docker_url = match env::var("DOCKER_URL") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    if jira_url.is_empty() || docker_url.is_empty() {
        println!("Missing webhook url");
        process::exit(-1);
    }

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
