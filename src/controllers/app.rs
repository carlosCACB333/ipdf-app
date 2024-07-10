use actix_web::web;
use crate::services::app::health;

pub fn routes() -> actix_web::Scope {
    web::scope("").service(health)
}
