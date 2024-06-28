use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use env_logger::Env;
use ipdf::controllers::pdf;
use log::info;
use std::env;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stage = env::var("STAGE").unwrap_or("development".to_string());
    if stage == "development" {
        dotenv().ok();
    }
    let port = u16::from_str_radix(&std::env::var("APP_PORT").unwrap(), 10).unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    HttpServer::new(move || {
        info!("Starting server on port {}", port);
        let api = web::scope("/api").service(pdf::routes());
        App::new()
            // .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(api)
            .service(web::scope("").service(home_page))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

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
