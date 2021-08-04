#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

mod user;
mod database;
mod error;

use rocket::{Rocket, Build};
use rocket::response::Debug;
use database::Db;

pub type WebResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Db::fairing())
        .attach(user::routes())
}
