use std::net::TcpListener;

use anyhow::Result;

use simple_http_server::handler::Server;
use simple_http_server::handler::static_page::StaticPageHandler;

// use anyhow::Result;
fn main() -> Result<()> {
    let home_handler = StaticPageHandler::new("assets/hello.html".into());
    let server = Server::default()
        .with_endpoint("/", &home_handler)
        .with_endpoint("/test", &home_handler);

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    server.listen(listener)
}
