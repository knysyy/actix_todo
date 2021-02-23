use actix_web::{dev::HttpServiceFactory, web, Responder};

use crate::models::{response::ResponseBody, user::User};

#[get("")]
async fn get_user() -> impl Responder {
    let login_user = json! {
    {
        "id": 1,
        "email": "test@test.test",
        "username": "test",
        "password": "****",
        "created_at": "2021-01-01T00:00:00",
        "updated_at": "2021-01-01T00:00:00"
    }
    };
    let user: User = serde_json::from_value(login_user).unwrap();
    ResponseBody::new(None, user).build()
}

pub fn config_route() -> impl HttpServiceFactory {
    web::scope("/users").service(get_user)
}
