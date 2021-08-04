use rocket::fairing::AdHoc;

use crate::actions::user_action;

pub fn index() -> AdHoc {
    AdHoc::on_ignite("user_route_index", |rocket| async {
        rocket.mount("/users", routes![
            user_action::index,
            user_action::store
        ])
    })
}
