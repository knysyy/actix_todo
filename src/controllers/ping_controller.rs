use actix_web::{dev::HttpServiceFactory, web, Responder};

use crate::models::response::{ErrorResponse, ResponseBody};

#[get("/ping")]
async fn ping() -> impl Responder {
    ResponseBody::new(None, "pong").build()
}

#[get("/badrequest")]
async fn bad_request() -> impl Responder {
    ErrorResponse::new(
        actix_web::http::StatusCode::BAD_REQUEST,
        "The request is invalid",
    )
    .build()
}

#[get("/internalservererror")]
async fn internal_server_error() -> impl Responder {
    ErrorResponse::new(
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error",
    )
    .build()
}

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[post("/json")]
async fn json(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}", info.name)
}

#[get("/path/{id}")]
async fn path(web::Path(id): web::Path<u32>) -> impl Responder {
    format!("id : {}", id)
}

pub fn config_route() -> impl HttpServiceFactory {
    web::scope("/ping")
        .service(ping)
        .service(bad_request)
        .service(internal_server_error)
        .service(json)
        .service(path)
}
