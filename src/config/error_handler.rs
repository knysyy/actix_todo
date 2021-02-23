use actix_web::{
    http,
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
    ResponseError,
};

use crate::error::CustomServiceError;

pub fn config_error_handlers<T>() -> ErrorHandlers<T> {
    ErrorHandlers::new()
        .handler(http::StatusCode::INTERNAL_SERVER_ERROR, |res| {
            Ok(ErrorHandlerResponse::Response(
                res.into_response(
                    CustomServiceError::InternalServerError
                        .error_response()
                        .into_body(),
                ),
            ))
        })
        .handler(http::StatusCode::BAD_REQUEST, |res| {
            Ok(ErrorHandlerResponse::Response(
                res.into_response(
                    CustomServiceError::BadRequest(String::from("The request is invalid"))
                        .error_response()
                        .into_body(),
                ),
            ))
        })
        .handler(http::StatusCode::NOT_FOUND, |res| {
            Ok(ErrorHandlerResponse::Response(res.into_response(
                CustomServiceError::NotFound.error_response().into_body(),
            )))
        })
}
