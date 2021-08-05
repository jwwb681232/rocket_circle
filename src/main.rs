#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;
//#[macro_use] extern crate serde_json;

mod user;
mod database;
mod error;

use rocket::{Rocket, Build};
//use rocket::response::Debug;
use database::Db;

pub type ApiResult<T> = Result<T, error::ApiError>;
pub type InternalResult<T> = Result<T, error::InternalError>;


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Db::fairing())
        .attach(user::routes())
}
