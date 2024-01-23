use std::str::FromStr;

use anyhow::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RequestMethod {
    GET
}

impl FromStr for RequestMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("GET") {
            Ok(RequestMethod::GET)
        } else {
            Err(Error::msg("The request line either doesn't start with an HTTP Method, or it isn't implemented"))
        }
    }
}

