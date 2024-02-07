use std::fmt::{Debug, Formatter};

use anyhow::Context;
use itertools::Itertools;

use crate::http::methods::RequestMethod;

#[derive(Clone)]
pub struct HttpRequest {
    pub method: RequestMethod,
    pub uri: String,
    body: Vec<String>,
}

impl TryFrom<Vec<String>> for HttpRequest {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> anyhow::Result<HttpRequest> {
        let (header_token, body_tokens) = value
            .split_first()
            .context(format!("The request was incomplete:{:?}", value))?;
        let (method_token, uri, _): (&str, &str, &str) = header_token
            .split_whitespace()
            .next_tuple()
            .context("Missing URI, or Version token")?;
        let method: RequestMethod = method_token.parse()?;
        let body = body_tokens.to_vec();
        Ok(HttpRequest {
            method,
            uri: uri.into(),
            body,
        })
    }
}

impl Debug for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HttpRequest(method={:?},uri={:?},body={:?})",
            self.method, self.uri, self.body
        )?;
        Ok(())
    }
}
