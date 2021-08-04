use rocket::fairing::AdHoc;

use crate::actions::auth_action;

pub fn index() -> AdHoc {
    AdHoc::on_ignite("auth_route_index", |rocket| async {
        rocket.mount("/auth", routes![
            auth_action::login
        ])
    })
}
