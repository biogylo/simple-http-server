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

#[derive(Debug, Clone, Copy)]
pub enum HttpStatusCode {
    Ok = 200,
    BadRequest = 400,
    MovedPermanently = 300,
    IoError = 123456,
}

impl From<&HttpStatusCode> for usize {
    fn from(value: &HttpStatusCode) -> Self {
        value.to_owned() as usize
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
