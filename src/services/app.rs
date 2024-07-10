use crate::{make_response, utils::pdf::create_pdf};
use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use std::env;

#[get("/")]
pub async fn home_page() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <html>
            <head>
                <title>IPDF API</title>
            </head>
            <body>
                <h1>IPDF API</h1>
                <p>Welcome to the IPDF API. This is a simple API that allows you to build PDFs.</p>
            </body>
        </html>
        "#,
    )
}

#[get("/health")]
pub async fn health() -> impl Responder {
    log::info!("Requesting health check");
    let base_url = env::var("APP_URL").unwrap();
    let text = "I'm alive!";
    let path = create_pdf(text);
    let url = format!("{}/{}", base_url, path);
    make_response!(json!({ "message": "I'm alive!" ,"url": url}))
}
