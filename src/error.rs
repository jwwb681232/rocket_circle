use thiserror::Error;
use std::io::Cursor;
use rocket::serde::json::Value;
use rocket::response::Responder;
use rocket::Request;
use rocket::http::Status;
use rocket::http::ContentType;

#[derive(Error, Debug)]
pub enum Error{
    #[error("Unauthorized")]
    Unauthorized(Value),
    #[error("Forbidden")]
    Forbidden(Value),
    #[error("Not Found")]
    NotFound(Value),
    #[error("Verification failed")]
    Validate(Value),
    #[error("Server Error")]
    InternalServerError,
}

impl<'a> Responder<'a, 'static> for Error {
    /*fn respond_to(self, _: &rocket::Request) -> Result<rocket::Response<'a>, Status> {
        match self {
            Error::Unauthorized(_) => {}
            Error::Forbidden(_) => {}
            Error::NotFound(_) => {}
            Error::Validate(_) => {}
            Error::InternalServerError => {}
        }
    }*/

    fn respond_to<'r>(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut body;
        let mut status;

        match self {
            Error::Unauthorized(_) => {body = "Unauthorized".to_string();status = Status::Unauthorized;}
            Error::Forbidden(_) => {body = "Unauthorized".to_string();status = Status::Forbidden;}
            Error::NotFound(e) => {body = e.to_string();status = Status::NotFound;}
            Error::Validate(_) => {body = "Unauthorized".to_string();status = Status::UnprocessableEntity;}
            Error::InternalServerError => {body = "Unauthorized".to_string();status = Status::InternalServerError;}
        }

        rocket::Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}
