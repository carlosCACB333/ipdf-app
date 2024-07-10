use crate::services::app::{health, home_page};
use actix_web::web;

pub fn routes() -> actix_web::Scope {
    web::scope("").service(health).service(home_page)
}
