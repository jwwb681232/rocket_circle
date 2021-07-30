#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

mod bootstrap;
mod schema;
mod routes;
mod actions;
mod models;
mod dto;

use rocket::{Rocket, Build};
use rocket::response::Debug;
use crate::bootstrap::database::Db;

pub type WebResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Db::fairing())
        .attach(routes::users::index())
}
