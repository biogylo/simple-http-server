use std::net::TcpListener;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use directories::UserDirs;

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

    /// Port
    #[arg(long)]
    logs_directory: Option<String>,
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
    let log_directory: PathBuf = match args.logs_directory {
        None => {
            // Use the users directory
            UserDirs::new()
                .with_context(|| "Unable to obtain home directory for default logs folder! Fatal!")?
                .home_dir()
                .to_path_buf()
                .join(".logs/simple-http-server/")
        }
        Some(directory_string) => PathBuf::from(directory_string),
    };

    if !&log_directory.exists() {
        std::fs::create_dir_all(&log_directory)
            .with_context(|| format!("Error creating logs directory in {:?}", &log_directory))?;
    }
    if !log_directory.is_dir() {
        Err(anyhow!(
            "The given logs directory argument is not a path! {:?}",
            &log_directory
        ))?;
    }

    let mut server: StaticWebsite = StaticWebsite::new(public_directory);
    let ip_address = format!("127.0.0.1:{}", args.listen_port);
    let listener = TcpListener::bind(&ip_address)?;
    println!("Listening on {}", ip_address);
    server.listen(listener, log_directory)
}
