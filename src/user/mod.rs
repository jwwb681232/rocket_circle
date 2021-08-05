pub mod model;
pub mod schema;
pub mod form;

use rocket::fairing::AdHoc;
//use chrono::{Local, TimeZone};
use rocket::serde::json::Json;
use crate::database::Db;
use crate::ApiResult;
use rocket::form::Form;
//use crate::error::Error;

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("user_route_index", |rocket| async {
        rocket.mount("/users", routes![
            index,create
        ])
    })
}


#[get("/")]
pub async fn index(db: Db) -> ApiResult<Json<Vec<model::User>>> {

    /*return Err(Error::NotFound(json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    })));*/

    let results = model::User::list(db).await.unwrap();

    /*for result in &results {
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).to_rfc2822().to_string());
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).to_rfc3339().to_string());
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).format("%Y-%m-%d %H:%M:%S").to_string());
    }*/

    Ok(Json(results))
}

#[post("/", data = "<request>")]
pub async fn create(db:Db,request:Form<form::User>)->ApiResult<String>{
    Ok(
        model::User::create(db, request).await.to_string()
    )
}
