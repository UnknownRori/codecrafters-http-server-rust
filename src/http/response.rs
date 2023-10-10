use core::fmt;

use super::{ContentType, HttpCode};

pub struct Response {
    http_code: HttpCode,
    content_type: Option<ContentType>,
    content_length: Option<usize>,
    body: String,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut response = "HTTP/1.1 ".to_owned();
        response.push_str(self.http_code.to_string().as_str());

        if let Some(content_type) = &self.content_type {
            response.push_str(content_type.to_string().as_str());
            let length = self.content_length.unwrap();
            response.push_str(format!("Content-Length: {}\r\n\r\n", length).as_ref());
            response.push_str(self.body.as_str());
        }

        response.push_str("\r\n");

        write!(f, "{}", response)?;
        Ok(())
    }
}

pub struct ResponseBuilder {
    http_code: Option<HttpCode>,
    content_type: Option<ContentType>,
    content_length: Option<usize>,
    body: Option<String>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            http_code: None,
            content_type: None,
            content_length: None,
            body: None,
        }
    }

    pub fn code(mut self, code: HttpCode) -> Self {
        self.http_code = Some(code);
        self
    }

    pub fn content(mut self, content: String, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self.content_length = Some(content.len());
        self.body = Some(content);
        self
    }
}

impl From<ResponseBuilder> for Response {
    fn from(value: ResponseBuilder) -> Self {
        Self {
            http_code: value.http_code.unwrap_or(HttpCode::Ok200),
            body: value.body.unwrap_or(String::new()),
            content_length: value.content_length,
            content_type: value.content_type,
        }
    }
}
