use actix_web::web;

use crate::services::pdfs::{
    delete_page::delete_pages, get_page_img::get_pages_img, join::join_pdfs,
};

pub fn routes() -> actix_web::Scope {
    let pdfs = web::scope("/pdfs")
        .service(join_pdfs)
        .service(delete_pages)
        .service(get_pages_img);
    pdfs
}
