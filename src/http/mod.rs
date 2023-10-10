pub mod app;
pub mod controller;
pub mod request;
pub mod response;
pub mod routes;

use core::fmt;

pub use app::App;
pub use controller::FnController;
pub use request::Request;
pub use response::Response;
pub use response::ResponseBuilder;
pub use routes::{Route, Routes};

#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    Get,
    Post,
}

pub enum HttpCode {
    Ok200,
    Ok201,
    Err404,
    Err500,
}

pub enum ContentType {
    TextPlain,
    OctetStream,
    None,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ContentType::TextPlain => "Content-Type: text/plain",
            ContentType::OctetStream => "Content-Type: application/octet-stream",
            ContentType::None => "",
        };

        write!(f, "{}\r\n", str)
    }
}

impl fmt::Display for HttpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            HttpCode::Ok200 => "200 Ok",
            HttpCode::Ok201 => "201 Created",
            HttpCode::Err404 => "404 Not Found",
            HttpCode::Err500 => "500 Internal Server Error",
        };

        write!(f, "{}\r\n", str)
    }
}
