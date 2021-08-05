use rocket::serde::json::Value;
use std::fmt::Formatter;
use rocket::response::Responder;
use rocket::http::Status;
use rocket::Request;
use std::io::Cursor;
use rocket::http::ContentType;

#[derive(Debug)]
pub struct InternalError;

pub enum Error {
    BusinessError {
        code: u32,
        errors: Value,
        message: &'static str,
    },
    InternalError(InternalError)
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "内部错误!!!")
    }
}

impl<'a> Responder<'a, 'static> for Error {

    fn respond_to<'r>(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let body = "testa";
        let status = Status::Ok;



        rocket::Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}

impl From<std::io::Error> for InternalError{
    fn from(_: std::io::Error) -> Self {
        Self
    }
}


impl<'a> Responder<'a, 'static> for InternalError {

    fn respond_to<'r>(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let body = "testa";
        let status = Status::InternalServerError;



        rocket::Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}
