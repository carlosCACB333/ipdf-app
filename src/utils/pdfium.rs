use pdfium_render::prelude::Pdfium;

pub fn pdf_ngine() -> Pdfium {
    Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./libs")).unwrap(),
    )
}
