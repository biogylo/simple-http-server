use std::{fs, io::BufReader, net::TcpListener};
use std::io::{BufRead, Write};
use std::net::TcpStream;

use simple_http_server::http::methods::RequestMethod::GET;
use simple_http_server::http::request::HttpRequest;
use simple_http_server::http::response::HttpResponse;
use simple_http_server::http::response::HttpStatusCode;

fn process_request(http_request: HttpRequest) -> HttpResponse {
    if http_request.method != GET || http_request.uri != "/" {
        return HttpResponse {
            version: Default::default(),
            status: HttpStatusCode::BadRequest,
            reason_phrase: "Unable to process".to_string(),
            header: Default::default(),
            body: Default::default(),
        };
    }

    let homepage = fs::read_to_string("assets/hello.html").unwrap();
    let len = homepage.len();
    HttpResponse {
        version: Default::default(),
        status: HttpStatusCode::Ok,
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