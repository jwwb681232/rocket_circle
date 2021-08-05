use rocket::serde::json::Value;
use std::fmt::Formatter;
use rocket::response::Responder;
use rocket::http::Status;
use std::io::Cursor;
use rocket::http::ContentType;
use diesel::result::DatabaseErrorKind;

/****************************** ApiError(暴露给外部的) *****************************/
#[derive(Debug)]
pub enum ApiError{
    BusinessError(Value),
    InternalError(InternalError)
}

impl std::fmt::Display for ApiError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BusinessError(e) => {write!(f, "逻辑错误: {:?}",e)}
            ApiError::InternalError(e) => {write!(f, "内部错误: {:?}",e)}
        }
    }
}


impl From<InternalError> for ApiError{
    fn from(e: InternalError) -> Self {
        ApiError::InternalError(InternalError::Database(e.to_string()))
    }
}

impl<'a> Responder<'a, 'static> for ApiError {

    fn respond_to<'r>(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let body;
        let status;

        match self {
            ApiError::BusinessError(_) => {
                body = "业务逻辑错误";
                status = Status::Ok;
            }
            ApiError::InternalError(_) => {
                body = "内部错误";
                status = Status::InternalServerError;
            }
        }

        rocket::Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}

/****************************** InternalError(内部错误) *****************************/
#[derive(Debug)]
pub enum InternalError{
    Io(String),
    Database(String),
    Serialize(String)
}

impl std::fmt::Display for InternalError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::Io(e) => {write!(f, "读写错误: {:?}",e)}
            InternalError::Database(e) => {write!(f, "数据库错误: {:?}",e)}
            InternalError::Serialize(e) => {write!(f, "序列化错误: {:?}",e)}
        }
    }
}

impl From<std::io::Error> for InternalError{
    fn from(e: std::io::Error) -> Self {
        InternalError::Io(e.to_string())
    }
}

impl From<diesel::result::Error> for InternalError{
    fn from(e: diesel::result::Error) -> Self {
        InternalError::Database(e.to_string())
    }
}

impl From<serde_json::Error> for InternalError{
    fn from(e: serde_json::Error) -> Self {
        InternalError::Serialize(e.to_string())
    }
}

impl From<diesel::result::DatabaseErrorKind> for InternalError{
    fn from(e: diesel::result::DatabaseErrorKind) -> Self {
        match e {
            DatabaseErrorKind::UniqueViolation => {InternalError::Database("Unique Violation".to_string())}
            DatabaseErrorKind::ForeignKeyViolation => {InternalError::Database("Foreign Key Violation".to_string())}
            DatabaseErrorKind::UnableToSendCommand => {InternalError::Database("DB Error".to_string())}
            DatabaseErrorKind::SerializationFailure => {InternalError::Database("DB Error".to_string())}
            DatabaseErrorKind::__Unknown => {InternalError::Database("DB Error".to_string())}
        }
    }
}


impl From<ApiError> for InternalError{
    fn from(e: ApiError) -> Self {
        InternalError::Database(e.to_string())
    }
}
