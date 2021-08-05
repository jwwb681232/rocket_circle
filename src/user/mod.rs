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
    let res = model::User::create(db, request).await?;

    Ok(res.to_string())
}
