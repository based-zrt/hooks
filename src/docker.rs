use actix_web::{post, Responder, HttpResponse};

#[post("/docker")]
async fn handle(req_body: String) -> impl Responder {
    HttpResponse::Accepted()
}