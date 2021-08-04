pub mod model;
pub mod schema;

use rocket::fairing::AdHoc;
use crate::user::model::User;
use chrono::{Local, TimeZone};
use rocket::serde::json::Json;
use crate::database::Db;

pub fn routes() -> AdHoc {
    AdHoc::on_ignite("user_route_index", |rocket| async {
        rocket.mount("/users", routes![
            index
        ])
    })
}


#[get("/")]
pub async fn index(db: Db) -> Json<Vec<User>> {

    let results = User::list(db).await.unwrap();

    for result in &results {
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).to_rfc2822().to_string());
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).to_rfc3339().to_string());
        println!("{:#?}", Local.timestamp(result.created_at.timestamp(), 0).format("%Y-%m-%d %H:%M:%S").to_string());
    }

    Json(results)
}
