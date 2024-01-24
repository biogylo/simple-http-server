use crate::http::request::HttpRequest;
use crate::http::response::{HttpResponse, HttpStatusCode};

pub fn default_request_handler(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        version: Default::default(),
        status: HttpStatusCode::BadRequest,
        reason_phrase: "The desired request was not found".to_string(),
        header: "".to_string(),
        body: "".to_string(),
    }
}
