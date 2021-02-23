use actix_web::{guard, web, HttpResponse};

use crate::controllers::*;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(auth_controller::config_route())
            .service(ping_controller::config_route())
            .service(user_controller::config_route())
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            ),
    );
}
