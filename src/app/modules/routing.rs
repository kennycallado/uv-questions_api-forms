use crate::app::modules::form::controller as form_controller;
use crate::app::modules::question::controller as question_controller;

// this file is needed to avoid public access to the modules
pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/question", question_controller::routes())
            .mount("/api/v1/form", form_controller::routes())
    })
}
