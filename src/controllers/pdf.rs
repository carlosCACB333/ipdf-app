use actix_web::web;

use crate::services::pdfs::{
    delete_pages::delete_pages, get_page_img::get_pages_img, merge::merge_pdfs,
};

pub fn routes() -> actix_web::Scope {
    let pdfs = web::scope("/pdf")
        .service(merge_pdfs)
        .service(delete_pages)
        .service(get_pages_img);
    pdfs
}
