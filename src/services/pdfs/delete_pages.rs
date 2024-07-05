use crate::{make_response, utils::pdf};
use actix_web::{post, web, Responder};
use serde_json::json;
use std::{env, path::Path};

#[derive(Debug, serde::Deserialize)]
struct DeletePageReq {
    url: String,
    pages: Vec<u16>,
}

#[post("/remove/pages")]
pub async fn delete_pages(body: web::Json<DeletePageReq>) -> impl Responder {
    // remove host from
    let base_url = env::var("APP_URL").unwrap();
    let url = body.url.clone().replace(&format!("{}/", base_url), "");
    let path = Path::new(&url);
    let pages = body.pages.clone();
    log::info!("Deleting pages: {:?} of file {:?}", pages, path);
    let output = pdf::delete_pages(path, pages);
    let url = format!("{}/{}", base_url, output);

    make_response!(json!({ "url": url }))
}
