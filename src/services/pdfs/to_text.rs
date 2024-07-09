use crate::{make_response, utils::pdf};
use actix_web::{post, web, Responder};
use serde_json::json;
use std::env;

#[derive(Debug, serde::Deserialize)]
struct PagesImgReq {
    url: String,
}

#[post("/to/text")]
pub async fn pdf_to_text(body: web::Json<PagesImgReq>) -> impl Responder {
    log::info!("Requesting estract text {:?}", body);
    let base_url = env::var("APP_URL").unwrap();
    let path = body.url.clone().replace(&format!("{}/", base_url), "");

    log::info!("Path: {}", path);
    let text = pdf::extract_text(&path);
    make_response!(json!({"text": text}))
}
