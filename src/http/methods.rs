use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RequestMethod {
    GET
}

impl FromStr for RequestMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("GET") {
            Ok(RequestMethod::GET)
        } else {
            Err("The request line either doesn't start with an HTTP Method, or it isn't implemented")
        }
    }
}

