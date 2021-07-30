use chrono::NaiveDateTime;
use serde::Serialize;


#[derive(Serialize, Debug, Queryable)]
pub struct UserIndex {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub remember_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
