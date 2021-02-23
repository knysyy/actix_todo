use actix_web::{dev::HttpServiceFactory, web, Responder, ResponseError};

use crate::{
    config::database::Pool,
    models::{
        response::ResponseBody,
        user::{LoginUser, SignUpUser},
    },
    services::auth_service,
};

#[post("/login")]
async fn login(login_user: web::Json<LoginUser>, pool: web::Data<Pool>) -> impl Responder {
    match auth_service::login(login_user.0, &pool) {
        Ok(message) => ResponseBody::new(None, message).build(),
        Err(err) => err.error_response(),
    }
}

#[post("/signup")]
async fn signup(signup_user: web::Json<SignUpUser>, pool: web::Data<Pool>) -> impl Responder {
    match auth_service::signup(signup_user.0, &pool) {
        Ok(message) => ResponseBody::new(None, message).build(),
        Err(err) => err.error_response(),
    }
}

pub fn config_route() -> impl HttpServiceFactory {
    web::scope("/auth").service(login).service(signup)
}
