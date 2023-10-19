mod docker;
mod jira;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(jira::handle)
            .service(docker::handle)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}