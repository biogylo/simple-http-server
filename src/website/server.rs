use crate::handler::default_handler::default_request_handler;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

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
pub trait Server {
    fn process_connection(&self, mut tcp_stream: TcpStream) -> anyhow::Result<()> {
        let request: HttpRequest = take_request_from_stream(&tcp_stream)?;
        let response: HttpResponse = self.serve(request);
        tcp_stream
            .write_all(response.to_string().as_bytes())
            .map_err(anyhow::Error::from)
    }

    fn listen(&self, tcp_listener: TcpListener) -> anyhow::Result<()> {
        for result in tcp_listener.incoming() {
            let _ = result
                .map_err(anyhow::Error::from)
                .and_then(|tcp_stream| self.process_connection(tcp_stream))
                .map_err(|e| println!("Error: {}", e));
        }
        Err(anyhow::Error::msg("listener.incoming() returned None?????"))
    }

    fn serve(&self, http_request: HttpRequest) -> HttpResponse;

    fn serve_error(&self, http_request: HttpRequest) -> HttpResponse {
        default_request_handler(http_request)
    }
}
