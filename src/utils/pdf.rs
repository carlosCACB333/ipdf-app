use image::ImageFormat;
use pdfium_render::prelude::*;
use std::{io::Cursor, path::Path, vec};

use super::pdfium::pdf_ngine;

pub fn join_pdfs(paths: Vec<&Path>) -> Result<Vec<u8>, PdfiumError> {
    let pdfium = pdf_ngine();
    let mut document = pdfium.create_new_pdf().unwrap();

    for path in paths {
        document
            .pages_mut()
            .append(&pdfium.load_pdf_from_file(path, None).unwrap())
            .unwrap();
    }

    let document = document.save_to_bytes().unwrap();
    Ok(document)
}

pub fn delete_pages(path: &Path, pages_num: Vec<u16>) -> Result<Vec<u8>, PdfiumError> {
    let pdfium = pdf_ngine();
    let document = pdfium.load_pdf_from_file(path, None).unwrap();
    let pages = document.pages();
    for page in pages_num {
        pages.get(page).unwrap().delete().unwrap();
    }
    let document = document.save_to_bytes().unwrap();
    Ok(document)
}

pub fn page_to_img(path: &Path) -> Result<Vec<Vec<u8>>, PdfiumError> {
    let pdfium = pdf_ngine();

    let document = pdfium.load_pdf_from_file(path, None).unwrap();
    let mut images = vec![];
    for page in document.pages().iter() {
        let image = page
            .render_with_config(
                &PdfRenderConfig::new()
                    .set_target_width(100)
                    .set_target_height(141),
            )
            .unwrap()
            .as_image();

        let mut buffer = Cursor::new(Vec::new());
        image.write_to(&mut buffer, ImageFormat::Png).unwrap();
        images.push(buffer.into_inner());
    }

    Ok(images)
}
