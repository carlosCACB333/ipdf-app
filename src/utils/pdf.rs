use pdfium_render::prelude::*;
use std::{fs::File, io::copy, path::Path, vec};
use uuid::Uuid;
use zip::{write::SimpleFileOptions, ZipWriter};

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
    let path = format!("uploads/{}.pdf", Uuid::new_v4());
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
    let path = format!("uploads/{}.pdf", Uuid::new_v4());
    document.save_to_file(&path).unwrap();
    path
}

pub fn page_to_img(path: &Path, pages: Option<Vec<u16>>, config: &PdfRenderConfig) -> Vec<String> {
    let pdfium = pdf_ngine();
    let document = pdfium.load_pdf_from_file(path, None).unwrap();

    let pages = pages.unwrap_or((1..document.pages().len() + 1).collect::<Vec<u16>>());
    let mut images = vec![];
    let doc_pages = document.pages();
    for page in pages {
        let doc_page = doc_pages.get(page - 1).unwrap();
        let image = doc_page.render_with_config(config).unwrap().as_image();

        // let mut buffer = Cursor::new(Vec::new());
        // image.write_to(&mut buffer, ImageFormat::Png).unwrap();

        let path = format!("uploads/imgs/{}-{}.png", Uuid::new_v4(), page);

        image.save(path.clone()).unwrap();
        images.push(path);
    }

    images
}

pub fn create_zip(paths: Vec<String>) -> String {
    let zip_path = format!("uploads/{}.zip", Uuid::new_v4());
    let mut zip = ZipWriter::new(File::create(&zip_path).unwrap());
    for path in paths {
        let options = SimpleFileOptions::default();
        let file_name = path.split('/').last().unwrap();
        zip.start_file(file_name, options).unwrap();
        let mut file = File::open(&path).unwrap();
        copy(&mut file, &mut zip).unwrap();
    }
    zip.finish().unwrap();
    zip_path
}
