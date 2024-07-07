use actix_web::web;

use crate::services::pdfs::{
    delete_pages::delete_pages, get_page_img::get_pages_img, merge::merge_pdfs, to_img::pdf_to_images, upload::{delete_file, upload_files}
};

pub fn routes() -> actix_web::Scope {
    web::scope("/pdf")
        .service(merge_pdfs)
        .service(delete_pages)
        .service(get_pages_img)
        .service(upload_files)
        .service(delete_file)
        .service(pdf_to_images)
}
