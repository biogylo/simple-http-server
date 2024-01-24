use std::collections::HashMap;
use std::ffi::OsString;

use crate::handler::default_handler::default_request_handler;
use crate::handler::handler::Handler;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::website::server::Server;

pub struct StaticWebsite {
    endpoints: HashMap<OsString, Box<dyn Handler>>,
}

impl StaticWebsite {
    pub fn default() -> Self {
        let server = StaticWebsite::new(&default_request_handler);
        server
    }
}

impl Server for StaticWebsite {
    fn serve(&self, http_request: HttpRequest) -> HttpResponse {
        if let Some(handler) = self.endpoints.get(&http_request.uri) {
            handler.handle(http_request)
        } else {
            self.serve_error(http_request)
        }
    }
}

impl StaticWebsite {
    pub fn new(default_handler: &dyn Handler) -> StaticWebsite {
        StaticWebsite {
            endpoints: HashMap::default(),
        }
    }

    pub fn with_endpoint(mut self, endpoint: OsString, handler: Box<dyn Handler>) -> StaticWebsite {
        self.endpoints.insert(endpoint, handler);
        self
    }
}
