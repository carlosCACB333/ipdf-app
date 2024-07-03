use crate::utils::{pdf, tools::make_response};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::Error, post, Responder};
use serde_json::json;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart()]
    files: Vec<TempFile>,
}

#[post("/merge")]
pub async fn merge_pdfs(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let paths = form.files.iter().map(|file| file.file.path()).collect();
    let path = pdf::merge_pdfs(paths).unwrap();
    Ok(make_response("SUCCED", json!({ "url": path })))
}
