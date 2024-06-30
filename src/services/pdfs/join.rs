use crate::utils::pdf;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::Error, post, HttpResponse, Responder};


#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart()]
    files: Vec<TempFile>,
}

#[post("/join")]
pub async fn join_pdfs(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let paths = form.files.iter().map(|file| file.file.path()).collect();

    let merged = pdf::join_pdfs(paths).unwrap();

    let response = HttpResponse::Ok()
        .content_type("application/pdf")
        .body(merged);

    Ok(response)
}
