use serde::Serialize;

#[derive(Debug,Serialize, FromForm,Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}
