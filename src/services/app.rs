use crate::{make_response, utils::pdf::create_pdf};
use actix_web::{get, Responder};
use serde_json::json;
use std::env;

#[get("/health")]
pub async fn health() -> impl Responder {
    log::info!("Requesting health check");
    let base_url = env::var("APP_URL").unwrap();
    let text = "I'm alive!";
    let path = create_pdf(text);
    let url = format!("{}/{}", base_url, path);
    make_response!(json!({ "message": "I'm alive!" ,"url": url}))
}
