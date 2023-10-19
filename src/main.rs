mod docker;
mod jira;

use std::env;

use actix_web::{App, HttpServer};
use actix_web_httpauth::extractors::bearer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .app_data(bearer::Config::default())
            .service(jira::handle)
            .service(docker::handle)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
