use pdfium_render::prelude::*;
use std::{env, path::Path, vec};

use super::pdfium::pdf_ngine;

pub fn merge_pdfs(paths: Vec<String>) -> Result<String, PdfiumError> {
    let pdfium = pdf_ngine();
    let mut document = pdfium.create_new_pdf().unwrap();

    for path in paths {
        document
            .pages_mut()
            .append(&pdfium.load_pdf_from_file(&path, None).unwrap())
            .unwrap();
    }
    let path = format!("uploads/{}.pdf", chrono::Local::now().timestamp_micros());
    document.save_to_file(&path).unwrap();

    Ok(path)
}

pub fn delete_pages(path: &Path, pages_num: Vec<u16>) -> String {
    let pdfium = pdf_ngine();
    let document = pdfium.load_pdf_from_file(path, None).unwrap();
    let pages = document.pages();
    for page in pages_num {
        pages.get(page - 1).unwrap().delete().unwrap();
    }
    // let document = document.save_to_bytes().unwrap();
    let path = format!("uploads/{}.pdf", chrono::Local::now().timestamp_micros());
    document.save_to_file(&path).unwrap();
    path
}

pub fn page_to_img(path: &Path, pages: Option<Vec<u16>>) -> Vec<String> {
    let pdfium = pdf_ngine();
    let document = pdfium.load_pdf_from_file(path, None).unwrap();

    let pages = pages.unwrap_or((1..document.pages().len() + 1).collect::<Vec<u16>>());
    let mut images = vec![];
    let doc_pages = document.pages();
    for page in pages {
        let doc_page = doc_pages.get(page - 1).unwrap();
        let image = doc_page
            .render_with_config(
                &PdfRenderConfig::new()
                    .set_target_width(100)
                    .set_target_height(141),
            )
            .unwrap()
            .as_image();

        // let mut buffer = Cursor::new(Vec::new());
        // image.write_to(&mut buffer, ImageFormat::Png).unwrap();

        let path = format!(
            "uploads/imgs/{}-{}.png",
            chrono::Local::now().timestamp_micros(),
            page
        );
        let base_url = env::var("APP_URL").unwrap();
        image.save(path.clone()).unwrap();
        images.push(format!("{}/{}", base_url, path));
    }

    images
}
