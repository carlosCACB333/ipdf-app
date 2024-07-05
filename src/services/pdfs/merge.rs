use std::env;

use crate::{make_response, utils::pdf};
use actix_web::{post, web, Responder};
use serde_json::json;

#[derive(Debug, serde::Deserialize)]
struct MergeReq {
    urls: Vec<String>,
}

#[post("/merge")]
pub async fn merge_pdfs(body: web::Json<MergeReq>) -> impl Responder {
    let paths = body
        .urls
        .iter()
        .map(|url| {
            let base_url = env::var("APP_URL").unwrap();
            let path = url.clone().replace(&format!("{}/", base_url), "");
            path
        })
        .collect::<Vec<String>>();

    let path = pdf::merge_pdfs(paths).unwrap();
    let url = format!("{}/{}", env::var("APP_URL").unwrap(), path);
    make_response!(json!({ "url": url }))
}
