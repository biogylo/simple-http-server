use crate::handler::Handler;
use crate::http::methods::RequestMethod::GET;
use crate::http::request::HttpRequest;
use crate::http::response::{HttpResponse, HttpStatusCode};
use std::fs;
use std::path::PathBuf;

pub struct StaticPageHandler {
    page: String,
}

impl StaticPageHandler {
    pub fn new(filepath: PathBuf) -> StaticPageHandler {
        StaticPageHandler {
            page: fs::read_to_string(filepath).unwrap(),
        }
    }
}
impl Handler for StaticPageHandler {
    fn handle(&self, http_request: HttpRequest) -> HttpResponse {
        if http_request.method != GET {
            return HttpResponse {
                version: Default::default(),
                status: HttpStatusCode::BadRequest,
                reason_phrase: "Unable to process".to_string(),
                header: Default::default(),
                body: Default::default(),
            };
        }

        HttpResponse {
            version: Default::default(),
            status: HttpStatusCode::Ok,
            reason_phrase: "OK".to_string(),
            header: format!("Content-Length: {}", self.page.len()),
            body: self.page.clone(),
        }
    }
}
