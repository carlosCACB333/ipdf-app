use crate::utils::pdf;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::Error, post, HttpResponse, Responder};
use std::path::Path;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart()]
    file: TempFile,
}

#[post("/pages/img")]
pub async fn get_pages_img(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let path: &Path = form.file.file.path();
    let img = pdf::page_to_img(path).unwrap();
    Ok(HttpResponse::Ok().json(img))
}
