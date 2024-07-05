use std::env;

use crate::{make_response, utils::tools::Status};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{delete, post, web::Path, Responder};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart()]
    file: Option<TempFile>,
}

#[derive(Debug, serde::Serialize)]
struct Data {
    url: String,
}

#[post("/upload")]
pub async fn upload_files(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    if form.file.is_none() {
        return make_response!(Status::FAILED, "No hay archivo para subir");
    }
    let file = form.file.unwrap();
    let file_name = chrono::Local::now().timestamp_micros().to_string();
    let path = format!("uploads/{}.pdf", file_name);
    file.file.persist(&path).unwrap();
    let base_url = env::var("APP_URL").unwrap();
    let url = format!("{}/{}", base_url, path);

    make_response!(Data { url })
}

#[delete("/delete/{file}")]
pub async fn delete_file(file: Path<String>) -> impl Responder {
    let path = format!("uploads/{}", file);
    std::fs::remove_file(path).unwrap_or_default();
    make_response!("Archivo eliminado")
}
