use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use anyhow::Context;
use itertools::enumerate;
use log;

use crate::http::request::HttpRequest;
use crate::http::response::{HttpResponse, HttpStatusCode};

impl HttpResponse {
    fn to_bytes(&self) -> Vec<u8> {
        let string_part = format!(
            "{} {} {}\r\n{}\r\n\r\n",
            self.version,
            usize::from(&self.status),
            self.reason_phrase,
            self.header
        );
        string_part.bytes().chain(self.body.clone()).collect()
    }
}

pub trait Server {
    fn take(&self, mut stream: &TcpStream) -> anyhow::Result<HttpRequest> {
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

    fn put(&self, mut stream: &TcpStream, response: &HttpResponse) -> anyhow::Result<()> {
        stream
            .write_all(&response.to_bytes())
            .map_err(anyhow::Error::from)
    }
    fn listen(&self, tcp_listener: TcpListener) -> anyhow::Result<()> {
        simple_logger::SimpleLogger::new().init().unwrap();
        for (incoming_id, result) in enumerate(tcp_listener.incoming()) {
            let result = result
                .map_err(anyhow::Error::from)
                .and_then(|stream| {
                    let socket_addr = stream
                        .peer_addr()
                        .with_context(|| "Unable to obtain peer address")?;
                    Ok((socket_addr, stream))
                })
                .and_then(|(socket_addr, stream)| {
                    let http_request = self.take(&stream).with_context(|| {
                        format!("Unable to take a valid request from {}", socket_addr)
                    })?;
                    Ok((socket_addr, stream, http_request))
                })
                .and_then(|(socket_addr, stream, http_request)| {
                    let http_response = self.serve(&http_request);
                    self.put(&stream, &http_response).with_context(|| {
                        format!("Unable to put a response into socket on {}", socket_addr)
                    })?;
                    let uri = (&http_request.uri).clone();
                    Ok((socket_addr, uri, http_response))
                });
            match result {
                Ok((socket_addr, uri, _response)) => {
                    log::debug!(
                        "Successfully served {} for uri {} to {}",
                        incoming_id,
                        uri,
                        socket_addr
                    )
                }
                Err(err) => {
                    log::debug!(
                        "Unable to serve request {} due to error: {}",
                        incoming_id,
                        err.to_string()
                    )
                }
            };
        }
        Err(anyhow::Error::msg("listener.incoming() returned None?????"))
    }
    fn serve(&self, http_request: &HttpRequest) -> HttpResponse;

    fn serve_error(&self, _http_request: &HttpRequest) -> HttpResponse {
        HttpResponse {
            version: Default::default(),
            status: HttpStatusCode::BadRequest,
            reason_phrase: "The desired request was not found".to_string(),
            header: "".to_string(),
            body: "".into(),
        }
    }
}
