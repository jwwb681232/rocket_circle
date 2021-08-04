use rocket::serde::json::Value;

pub enum Error{
    Unauthorized(Value),
    Forbidden(Value),
    NotFound(Value),
}
