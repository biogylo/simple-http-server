use std::{fs, io::BufReader, net::TcpListener};
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::str::FromStr;

use itertools::Itertools;

use crate::RequestMethod::GET;

impl TryFrom<Vec<String>> for HttpRequest {
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<HttpRequest, &'static str> {
        let (header_token, body_tokens) = value.split_first().ok_or("The request was incomplete")?;
        let (method_token, uri, version): (&str, &str, &str) = header_token.split_whitespace().next_tuple().ok_or("Missing URI, or Version token")?;
        let method: RequestMethod = method_token.parse()?;
        let body = body_tokens.iter().cloned().collect();
        Ok(HttpRequest {
            method,
            uri: uri.to_string(),
            body,
        })
    }
}

fn process_request(http_request: HttpRequest) -> HttpResponse {
    if http_request.method != GET || http_request.uri != "/" {
        return HttpResponse {
            version: Default::default(),
            status: HttpStatusCode::BAD_REQUEST,
            reason_phrase: "Unable to process".to_string(),
            header: Default::default(),
            body: Default::default(),
        };
    }

    let homepage = fs::read_to_string("assets/hello.html").unwrap();
    let len = homepage.len();
    HttpResponse {
        version: Default::default(),
        status: HttpStatusCode::OK,
        reason_phrase: "OK".to_string(),
        header: format!("Content-Length: {len}"),
        body: homepage,
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), &'static str> {
    let buf_reader = BufReader::new(&mut stream);
    let http_lines: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()).collect();
    let request: HttpRequest = http_lines.try_into()?;
    let response = process_request(request).to_string();

    stream.write_all(response.as_bytes())
        .map_err(|_| "Unable to completely write response!")
        .map(|_| ())
    // Ok(response)
}


// use anyhow::Result;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let request = stream
            .map_err(|_| "TCP IO error")
            .and_then(handle_connection);
        match request {
            Ok(request) => println!("{:?}", request),
            Err(error) => println!("{}", error),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RequestMethod {
    GET
}

impl FromStr for RequestMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("GET") {
            Ok(GET)
        } else {
            Err("The request line either doesn't start with an HTTP Method, or it isn't implemented")
        }
    }
}

pub struct HttpRequest {
    method: RequestMethod,
    uri: String,
    body: Vec<String>,
}


impl Debug for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpRequest(method={:?},uri={:?},body={:?})", self.method, self.uri, self.body)?;
        Ok(())
    }
}

struct HttpVersion(String);

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion { 0: "HTTP/1.1".to_string() }
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum HttpStatusCode {
    OK,
    BAD_REQUEST,
}

impl From<&HttpStatusCode> for usize {
    fn from(value: &HttpStatusCode) -> Self {
        match value {
            HttpStatusCode::OK => 200,
            HttpStatusCode::BAD_REQUEST => 400,
        }
    }
}

pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatusCode,
    pub reason_phrase: String,
    pub header: String,
    pub body: String,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let status: usize = (&self.status).into();
        write!(f, "{} {} {}\r\n{}\r\n\r\n{}", self.version, status, self.reason_phrase, self.header, self.body)
    }
}