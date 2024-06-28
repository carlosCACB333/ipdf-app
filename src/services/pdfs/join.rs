use crate::utils::pdf;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::Error, post, HttpResponse, Responder};
use lopdf::Document;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart()]
    files: Vec<TempFile>,
}

#[post("/join")]
pub async fn join_pdfs(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let mut pdfs: Vec<Document> = vec![];
    for file in form.files {
        let doc = Document::load(file.file.path()).unwrap();
        pdfs.push(doc);
    }

    let mut merged: Document = pdf::join_pdfs(pdfs).unwrap();

    let mut buffer = Vec::new();
    merged.save_to(&mut buffer).unwrap();

    let response = HttpResponse::Ok()
        .content_type("application/pdf")
        .body(buffer);

    Ok(response)
}
