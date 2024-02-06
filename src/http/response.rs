use std::fmt::{Display, Formatter};

pub struct HttpVersion(String);

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion("HTTP/1.1".to_string())
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum HttpStatusCode {
    Ok,
    BadRequest,
}

impl From<&HttpStatusCode> for usize {
    fn from(value: &HttpStatusCode) -> Self {
        match value {
            HttpStatusCode::Ok => 200,
            HttpStatusCode::BadRequest => 400,
        }
    }
}

pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: HttpStatusCode,
    pub reason_phrase: String,
    pub header: String,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn from_page(body: &[u8]) -> Self {
        Self {
            version: Default::default(),
            status: HttpStatusCode::Ok,
            reason_phrase: "OK".to_string(),
            header: format!("Content-Length: {}", body.len()),
            body: body.to_vec(),
        }
    }
}
