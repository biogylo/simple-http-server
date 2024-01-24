use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;

pub trait Handler {
    fn handle(&self, http_request: HttpRequest) -> HttpResponse;
}

impl<F> Handler for F
where
    F: Fn(HttpRequest) -> HttpResponse,
{
    fn handle(&self, http_request: HttpRequest) -> HttpResponse {
        self(http_request)
    }
}
