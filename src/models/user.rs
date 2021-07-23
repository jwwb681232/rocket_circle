use chrono::NaiveDateTime;
use serde::Serialize;
use crate::bootstrap::helper::NaiveDatetimeLocal;

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

impl UserIndex {
    pub fn serialize_local(&self) -> Self {
        UserIndex {
            id: self.id,
            name: self.name.to_string(),
            email: self.name.to_string(),
            email_verified_at: self.email_verified_at,
            password: self.password.to_string(),
            remember_token: self.remember_token.to_string(),
            created_at: self.created_at.serialize_local(),
            updated_at: self.created_at.serialize_local(),
        }
    }
}
