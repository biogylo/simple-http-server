use std::net::TcpListener;
use std::path::Path;

use anyhow::Result;

use simple_http_server::website::server::Server;
use simple_http_server::website::static_website::StaticWebsite;

pub enum ServerChoice {
    Static,
    Hyperlinked,
}

// use anyhow::Result;
fn main() -> Result<()> {
    let maybe_assets_directory = std::env::args().nth(1);

    let mut server: StaticWebsite = match maybe_assets_directory {
        None => StaticWebsite::default(),
        Some(assets_directory) => StaticWebsite::from_assets(Path::new(&assets_directory))?,
    };
    let ip_address = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip_address)?;
    println!("Listening on {}", ip_address);
    server.listen(listener)
}
