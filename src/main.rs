use std::net::TcpListener;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;

use simple_http_server::website::server::Server;
use simple_http_server::website::static_website::StaticWebsite;

pub enum ServerChoice {
    Static,
    Hyperlinked,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Public assets directory
    #[arg(short, long)]
    public_directory: String,

    /// Port
    #[arg(short, long)]
    listen_port: usize,
}

// use anyhow::Result;
fn main() -> Result<()> {
    let args = Args::parse();
    let public_directory = PathBuf::from(args.public_directory);

    if !public_directory.exists() {
        Err(anyhow!(
            "The given public asset directory doesn't exist: {:?}",
            public_directory
        ))?;
    }

    let mut server: StaticWebsite = StaticWebsite::new(public_directory);
    let ip_address = format!("127.0.0.1:{}", args.listen_port);
    let listener = TcpListener::bind(&ip_address)?;
    println!("Listening on {}", ip_address);
    server.listen(listener)
}
