use crate::{
    make_response,
    utils::pdf::{self, create_zip},
};
use actix_web::{post, web, Responder};
use pdfium_render::prelude::PdfRenderConfig;
use serde_json::json;
use std::{env, path::Path};

#[derive(Debug, serde::Deserialize)]
struct PagesImgReq {
    url: String,
}

#[post("/to/images")]
pub async fn pdf_to_images(body: web::Json<PagesImgReq>) -> impl Responder {
    log::info!("Requesting pages images{:?}", body);
    let base_url = env::var("APP_URL").unwrap();
    let path = body.url.clone().replace(&format!("{}/", base_url), "");
    let path: &Path = Path::new(&path);

    log::info!("Getting pages images of file {:?}", path);
    let config = &PdfRenderConfig::new();
    let urls = pdf::page_to_img(path, None, config);
    let path = create_zip(urls.clone());
    let url = format!("{}/{}", base_url, path);
    let urls = urls
        .iter()
        .map(|img| format!("{}/{}", base_url, img))
        .collect::<Vec<String>>();
    make_response!(json!({ "zip": url,"urls":urls}))
}
