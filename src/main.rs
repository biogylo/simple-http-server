use std::net::TcpListener;

use anyhow::Result;

use simple_http_server::website::server::Server;
use simple_http_server::website::static_website::StaticWebsite;

pub enum ServerChoice {
    Static,
    Hyperlinked,
}

// use anyhow::Result;
fn main() -> Result<()> {
    let server: StaticWebsite = StaticWebsite::default();
    let ip_address = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip_address)?;
    println!("Listening on {}", ip_address);
    server.listen(listener)
}
