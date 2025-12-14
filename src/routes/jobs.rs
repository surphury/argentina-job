use actix_web::{HttpResponse, Responder, get};

#[get("/")]
async fn jobs() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
async fn job() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
