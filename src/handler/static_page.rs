use std::fs;
use std::path::PathBuf;

use crate::handler::handler::Handler;
use crate::http::methods::RequestMethod::GET;
use crate::http::request::HttpRequest;
use crate::http::response::{HttpResponse, HttpStatusCode};

pub struct StaticPageHandler {
    page: String,
}

impl StaticPageHandler {
    pub const fn new(page_html_string: String) -> StaticPageHandler {
        StaticPageHandler {
            page: page_html_string,
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
            body: self.page.to_string(),
        }
    }
}
