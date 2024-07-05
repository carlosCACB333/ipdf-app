use std::{env, path::Path};
use crate::{make_response, utils::pdf};
use actix_web::{post, web, Responder};

#[derive(Debug, serde::Deserialize)]
struct PagesImgReq {
    url: String,
    pages: Option<Vec<u16>>,
}

#[post("/pages/images")]
pub async fn get_pages_img(body: web::Json<PagesImgReq>) -> impl Responder {
    log::info!("Requesting pages images{:?}", body);
    let base_url = env::var("APP_URL").unwrap();
    let path = body.url.clone().replace(&format!("{}/", base_url), "");
    let path: &Path = Path::new(&path);
    let pages = body.pages.clone();
    log::info!("Getting pages images: {:?} of file {:?}", pages, path);
    let urls = pdf::page_to_img(path, pages);
    make_response!(urls)
}
