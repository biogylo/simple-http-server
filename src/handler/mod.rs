use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};




use crate::http::request::HttpRequest;
use crate::http::response::{HttpResponse, HttpStatusCode};

pub mod static_page;

pub trait Handler {
    fn handle(&self, http_request: HttpRequest) -> HttpResponse;
}

impl<F> Handler for F
    where F: Fn(HttpRequest) -> HttpResponse {
    fn handle(&self, http_request: HttpRequest) -> HttpResponse {
        self(http_request)
    }
}
// pub struct Handler(Box<dyn Fn(request_body:) -> HttpResponse>);

pub struct Server<'a> {
    endpoints: HashMap<String, &'a dyn Handler>,
}

fn default_request_handler(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        version: Default::default(),
        status: HttpStatusCode::BadRequest,
        reason_phrase: "The desired request was not found".to_string(),
        header: "".to_string(),
        body: "".to_string(),
    }
}

impl Default for Server<'_> {
    fn default() -> Self {
        let server = Server::new(&default_request_handler);
        server
    }
}


fn take_request_from_stream(mut stream: &TcpStream) -> anyhow::Result<HttpRequest> {
    let buf_reader = BufReader::new(&mut stream);
    let mut http_lines: Vec<String> = Default::default();
    for line_result in buf_reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            break;
        } else {
            http_lines.push(line);
        }
    }
    http_lines.try_into()
}

impl<'a> Server<'a> {
    pub fn process_connection(&self, mut tcp_stream: TcpStream) -> anyhow::Result<()> {
        let request: HttpRequest = take_request_from_stream(&tcp_stream)?;
        let response: HttpResponse = self.handle(request);
        tcp_stream.write_all(response.to_string().as_bytes())
            .map_err(anyhow::Error::from)
    }
    pub fn listen(self, tcp_listener: TcpListener) -> anyhow::Result<()> {
        for result in tcp_listener.incoming() {
            let _ = result.map_err(anyhow::Error::from)
                .and_then(|tcp_stream| self.process_connection(tcp_stream))
                .map_err(|e| println!("Error: {}", e));
        }
        Err(anyhow::Error::msg("listener.incoming() returned None?????"))
    }
    pub fn new(default_handler: &dyn Handler) -> Server {
        let the_server = Server {
            endpoints: HashMap::default(),
        };
        the_server.with_endpoint("", default_handler)
    }

    pub fn handle(&self, http_request: HttpRequest) -> HttpResponse {
        let handler = if let Some(handler) = self.endpoints.get(http_request.uri.as_str()) {
            handler
        } else {
            self.endpoints.get("").expect("This is an invariant of this object")
        };
        handler.handle(http_request)
    }

    pub fn with_endpoint(mut self, endpoint: &'a str, handler: &'a dyn Handler) -> Server<'a> {
        let endpoint = endpoint.to_string();
        self.endpoints.insert(endpoint, handler);
        self
    }
}
