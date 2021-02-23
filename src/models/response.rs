use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T>
where
    T: Serialize,
{
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
}

impl<T> ResponseBody<T>
where
    T: Serialize,
{
    pub fn new(message: Option<&str>, result: T) -> ResponseBody<T> {
        ResponseBody {
            message: match message {
                Some(msg) => msg.to_string(),
                None => String::from("OK"),
            },
            result: Some(result),
        }
    }

    pub fn build(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::OK)
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(self)
    }
}

pub struct ErrorResponse {
    pub status: StatusCode,
    pub body: ResponseBody<Option<String>>,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: &str) -> ErrorResponse {
        ErrorResponse {
            status,
            body: ResponseBody {
                message: message.to_string(),
                result: None,
            },
        }
    }

    pub fn build(&self) -> HttpResponse {
        HttpResponse::build(self.status)
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(&self.body)
    }
}
