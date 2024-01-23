use std::fmt::{Display, Formatter};

pub struct HttpVersion(String);

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion {
            0: "HTTP/1.1".to_string(),
        }
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
    pub body: String,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let status: usize = (&self.status).into();
        write!(
            f,
            "{} {} {}\r\n{}\r\n\r\n{}",
            self.version, status, self.reason_phrase, self.header, self.body
        )
    }
}
