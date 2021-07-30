use chrono::{NaiveDateTime, Utc, Local, TimeZone};
use crate::bootstrap::database::Db;
use rocket::form::Form;
use rocket::serde::json::Json;
use serde::Serialize;
use crate::schema::users;
use diesel::prelude::*;
use crate::dto::user_dot::UserIndex;
use crate::WebResult;

#[get("/")]
pub async fn index(db: Db) -> WebResult<Json<Vec<UserIndex>>> {

    let results: Vec<UserIndex> = db.run(move |conn| {
        crate::schema::users::dsl::users.limit(15).load::<UserIndex>(conn)
    }).await?;

    for result in &results {
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).to_rfc2822().to_string());
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).to_rfc3339().to_string());
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).format("%Y-%m-%d %H:%M:%S").to_string());
    }

    Ok(Json(results))
}


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

    let req = request.clone();

    let result: usize = db.run(move |conn| {

        let new_user = NewUser{
            name: req.name,
            email: req.email,
            email_verified_at: None,
            password:  req.password,
            created_at: Utc::now().naive_local(),
            updated_at: Utc::now().naive_local(),
        };

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
