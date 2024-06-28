use actix_web::web;

use crate::services::pdfs::join::join_pdfs;

pub fn routes() -> actix_web::Scope {
    let pdfs = web::scope("/pdfs").service(join_pdfs);
    pdfs
}
