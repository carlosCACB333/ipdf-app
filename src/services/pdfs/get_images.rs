use crate::{
    make_response,
    utils::pdf::{self, create_zip},
};
use actix_web::{post, web, Responder};
use serde_json::json;
use std::env;

#[derive(Debug, serde::Deserialize)]
struct PagesImgReq {
    url: String,
}

#[post("/get/images")]
pub async fn pdf_get_images(body: web::Json<PagesImgReq>) -> impl Responder {
    log::info!("Requesting get images {:?}", body);
    let base_url = env::var("APP_URL").unwrap();
    let path = body.url.clone().replace(&format!("{}/", base_url), "");

    log::info!("Path: {}", path);
    let path_imgs = pdf::extract_images(&path);
    let path_zip = create_zip(path_imgs.clone());
    let url_zip = format!("{}/{}", base_url, path_zip);
    let urls_imgs = path_imgs
        .iter()
        .map(|img| format!("{}/{}", base_url, img))
        .collect::<Vec<String>>();
    make_response!(json!({"urls": urls_imgs,"zip": url_zip}))
}
