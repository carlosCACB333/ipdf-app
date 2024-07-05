use std::{fs, time::SystemTime};
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub enum Status {
    SUCCESS,
    FAILED,
}

#[derive(Serialize)]
pub struct Response<'a, T: Serialize> {
    status: Status,

    data: Option<T>,
    message: &'a str,
}

pub fn make_res<T: Serialize>(status: Status, message: &str, data: T) -> HttpResponse {
    let response = Response {
        status,
        message,
        data: Some(data),
    };

    match status {
        Status::SUCCESS => HttpResponse::Ok().json(response),
        Status::FAILED => HttpResponse::BadRequest().json(response),
    }
}

pub fn delete_old_files(path: &str, after_sec: u64) {
    let files = fs::read_dir(path);
    if files.is_err() {
        return;
    }
    for entry in files.unwrap().flat_map(Result::ok) {
        let path = entry.path();
        let metadata = fs::metadata(&path);
        if metadata.is_err() {
            continue;
        }
        let metadata = metadata.unwrap();

        if metadata.is_dir() {
            continue;
        }

        let created = metadata.created();
        if created.is_err() {
            continue;
        }

        let now = SystemTime::now();
        let duration = now.duration_since(created.unwrap());
        if duration.is_err() {
            continue;
        }

        if duration.unwrap().as_secs() > after_sec {
            fs::remove_file(&path).unwrap();
        }
    }
}
