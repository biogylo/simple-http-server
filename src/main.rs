use std::net::TcpListener;

use anyhow::Result;

use simple_http_server::handler::static_page::StaticPageHandler;
use simple_http_server::website::hyperlink_website::build_hyperlinked_website;
use simple_http_server::website::server::Server;
use simple_http_server::website::static_website::StaticWebsite;

use crate::ServerChoice::{Hyperlinked, Static};

enum ServerChoice {
    Static,
    Hyperlinked,
}

fn get_server(choice: ServerChoice) -> Box<dyn Server> {
    match choice {
        ServerChoice::Static => {
            let handler = StaticPageHandler::new(include_str!("../assets/hello.html").to_string());
            Box::new(StaticWebsite::default().with_endpoint("/".into(), Box::new(handler)))
        }
        ServerChoice::Hyperlinked => Box::new(
            build_hyperlinked_website("assets/portfolio/".into()).expect("This should work"),
        ),
    }
}

// use anyhow::Result;
fn main() -> Result<()> {
    let home_handler = StaticPageHandler::new(include_str!("../assets/hello.html").to_string());
    let server: Box<dyn Server> = get_server(Hyperlinked);

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    server.listen(listener)
}
