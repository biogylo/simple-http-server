use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use itertools::enumerate;
use log;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

use crate::http::request::HttpRequest;
use crate::http::response::{HttpResponse, HttpStatusCode};

#[derive(Debug, Serialize, Deserialize)]
struct ServerRecord {
    unix_timestamp: f64,
    exchange_duration_seconds: f64,
    socket_address: String,
    requested_uri: String,
    response_status: Option<usize>,
    description: String,
}

enum ExchangeSummary {
    IoError(anyhow::Error),
    UnableToParseRequest((SocketAddr, anyhow::Error)),
    ErrorServing((SocketAddr, HttpRequest, anyhow::Error)),
    Served((SocketAddr, HttpRequest, HttpResponse)),
}

fn unix_timestamp_from(system_time: SystemTime) -> f64 {
    system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64()
}

impl Display for ExchangeSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangeSummary::IoError(err) => write!(
                f,
                "Unable to serve request, due to IO Error: {}",
                err.to_string()
            ),
            ExchangeSummary::UnableToParseRequest((socket_addr, err)) => write!(
                f,
                "Unable to serve request to address {} since it wasn't parseable, error: {}",
                socket_addr.to_string(),
                err.to_string()
            ),
            ExchangeSummary::ErrorServing((socket_addr, http_request, err)) => write!(
                f,
                "Unable to serve request to address {} for uri {:?}, due to an error: {}",
                socket_addr.to_string(),
                http_request.uri,
                err.to_string()
            ),
            ExchangeSummary::Served((socket_addr, http_request, http_response)) => write!(
                f,
                "Successfully served request to address {} for uri {:?}, with response status {}",
                socket_addr.to_string(),
                http_request.uri,
                http_response.status as usize
            ),
        }
    }
}

impl ServerRecord {
    fn from_exchange_summary(
        system_time: SystemTime,
        duration: Duration,
        exchange_summary: ExchangeSummary,
    ) -> ServerRecord {
        let unix_timestamp = unix_timestamp_from(system_time);
        let description = exchange_summary.to_string();
        let exchange_duration_seconds = duration.as_secs_f64();
        match exchange_summary {
            ExchangeSummary::IoError(_) => ServerRecord {
                unix_timestamp,
                exchange_duration_seconds,
                socket_address: "".to_string(),
                requested_uri: "".to_string(),
                response_status: None,
                description,
            },
            ExchangeSummary::UnableToParseRequest((socket_addr, _)) => ServerRecord {
                unix_timestamp,
                exchange_duration_seconds,
                socket_address: socket_addr.to_string(),
                requested_uri: "".to_string(),
                response_status: None,
                description,
            },
            ExchangeSummary::ErrorServing((socket_addr, http_request, _)) => ServerRecord {
                unix_timestamp,
                exchange_duration_seconds,
                socket_address: socket_addr.to_string(),
                requested_uri: http_request.uri,
                response_status: None,
                description,
            },
            ExchangeSummary::Served((socket_addr, http_request, http_response)) => ServerRecord {
                unix_timestamp,
                exchange_duration_seconds,
                socket_address: socket_addr.to_string(),
                requested_uri: http_request.uri,
                response_status: Some(http_response.status as usize),
                description,
            },
        }
    }
}

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
        // Get the current local timestamp
        let timestamp_string = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let csv_filename = format!("simple_http_server_{}.csv", timestamp_string);
        let csv_path = Path::new("/var/log/myserver/").join(csv_filename);
        let _log_filename = format!("simple_http_server_{}.log", timestamp_string);
        SimpleLogger::new().init().unwrap();
        let mut csv_writer = csv::Writer::from_path(&csv_path).with_context(|| {
            format!(
                "Unable to initialize the csv writer from path {:?}",
                csv_path
            )
        })?;

        for (incoming_id, result) in enumerate(tcp_listener.incoming()) {
            let start_time = Instant::now();
            let result = result
                .map_err(|err| ExchangeSummary::IoError(anyhow::Error::from(err)))
                .and_then(|stream| {
                    let socket_addr = stream
                        .peer_addr()
                        .with_context(|| "Unable to obtain peer address")
                        .map_err(|err| ExchangeSummary::IoError(anyhow::Error::from(err)))?;
                    Ok((socket_addr, stream))
                })
                .and_then(|(socket_addr, stream)| {
                    let http_request = self
                        .take(&stream)
                        .with_context(|| {
                            format!("Unable to take a valid request from {}", socket_addr)
                        })
                        .map_err(|err| {
                            ExchangeSummary::UnableToParseRequest((
                                socket_addr,
                                anyhow::Error::from(err),
                            ))
                        })?;
                    Ok((socket_addr, stream, http_request))
                })
                .and_then(|(socket_addr, stream, http_request)| {
                    let http_response = self.serve(&http_request);

                    self.put(&stream, &http_response)
                        .with_context(|| {
                            format!("Unable to put a response into socket on {}", socket_addr)
                        })
                        .map_err(|err| {
                            ExchangeSummary::ErrorServing((
                                socket_addr,
                                http_request.clone(),
                                anyhow::Error::from(err),
                            ))
                        })?;
                    Ok(ExchangeSummary::Served((
                        socket_addr,
                        http_request,
                        http_response,
                    )))
                });
            let (Ok(exchange_summary) | Err(exchange_summary)) = result;
            let duration = Instant::now().duration_since(start_time);
            log::debug!(
                "Request #{} - {}",
                incoming_id,
                &exchange_summary.to_string()
            );
            let end_time = SystemTime::now();
            let server_record: ServerRecord =
                ServerRecord::from_exchange_summary(end_time, duration, exchange_summary);
            csv_writer.serialize(server_record).with_context(|| {
                format!("Unable to add row to CSV! Aborting server due to lack of function!")
            })?;
            csv_writer.flush().with_context(|| {
                format!("Unable to flush to CSV! Aborting server due to lack of function!")
            })?;
        }
        Err(anyhow::Error::msg("listener.incoming() returned None?????"))
    }
    fn serve(&self, http_request: &HttpRequest) -> HttpResponse;

    fn redirect_to_index(&self, _http_request: &HttpRequest) -> HttpResponse {
        HttpResponse {
            version: Default::default(),
            status: HttpStatusCode::MovedPermanently,
            reason_phrase: "Moved permanently".to_string(),
            header: "Location: /index.html".to_string(),
            body: "".into(),
        }
    }
}
