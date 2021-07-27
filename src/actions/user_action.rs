use chrono::{NaiveDateTime, Utc};
use crate::bootstrap::database::Db;
use rocket::form::Form;
use rocket::serde::json::Json;
use serde::Serialize;
use crate::schema::users;
use diesel::prelude::*;


#[derive(Serialize)]
struct StoreResponse {
    code: u16,
    data: Option<Vec<i32>>,
    message: String,
}

#[derive(Debug, FromForm,Clone)]
pub struct StoreRequest {
    name: String,
    email: String,
    password: String,
    #[field(validate = eq(self.password.clone()))]
    password_confirm: String,
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser {
    name: String,
    email: String,
    email_verified_at:Option<NaiveDateTime>,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[post("/", data = "<request>")]
pub async fn store(db: Db, request: Form<StoreRequest>) -> Json<StoreResponse> {

    let new_user = NewUser{
        name: request.name.clone(),
        email: request.email.clone(),
        email_verified_at: None,
        password:  request.password.clone(),
        created_at: Utc::now().naive_local(),
        updated_at: Utc::now().naive_local(),
    };

    let result: usize = db.run(move |conn| {

        diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .execute(conn).unwrap()

    }).await;

    Json(StoreResponse {
        code: 0,
        data: None,
        message: result.to_string(),
    })
}
