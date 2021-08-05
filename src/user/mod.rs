pub mod model;
pub mod schema;

use rocket::fairing::AdHoc;
use crate::user::model::User;
use chrono::{Local, TimeZone};
use rocket::serde::json::Json;
use crate::database::Db;
use crate::{ApiResult, InternalResult};
use crate::error::{Error, InternalError};

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("user_route_index", |rocket| async {
        rocket.mount("/users", routes![
            index
        ])
    })
}


#[get("/")]
pub async fn index(db: Db) -> InternalResult<&'static str> {
    let res = std::fs::read_to_string("address.txt")?;
    println!("{:?}",res);


    Ok("test")
}
