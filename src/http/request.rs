use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::http::methods::RequestMethod;

pub struct HttpRequest {
    pub method: RequestMethod,
    pub uri: String,
    body: Vec<String>,
}


impl TryFrom<Vec<String>> for HttpRequest {
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<HttpRequest, &'static str> {
        let (header_token, body_tokens) = value.split_first().ok_or("The request was incomplete")?;
        let (method_token, uri, _): (&str, &str, &str) = header_token.split_whitespace().next_tuple().ok_or("Missing URI, or Version token")?;
        let method: RequestMethod = method_token.parse()?;
        let body = body_tokens.iter().cloned().collect();
        Ok(HttpRequest {
            method,
            uri: uri.to_string(),
            body,
        })
    }
}
impl Debug for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpRequest(method={:?},uri={:?},body={:?})", self.method, self.uri, self.body)?;
        Ok(())
    }
}
