use crate::utils::pdf;
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{http::Error, post, HttpResponse, Responder};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart()]
    file: TempFile,

    pages: Text<String>,
}

#[post("/delete-pages")]
pub async fn delete_pages(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let file = form.file;

    let pages = form.pages.parse::<String>().unwrap();
    let pages = pages
        .split(",")
        .map(|page| page.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    log::info!("Deleting pages: {:?}", pages);

    let output = pdf::delete_pages(file.file.path(), pages).unwrap();

    let response = HttpResponse::Ok()
        .content_type("application/pdf")
        .body(output);

    Ok(response)
}
