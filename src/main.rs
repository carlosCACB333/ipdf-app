use actix_cors::Cors;
use actix_files::{self, Files};
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use env_logger::Env;
use ipdf::controllers::pdf;
use ipdf::utils::tools::delete_old_files;
use std::env;
use std::thread;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stage = env::var("STAGE").unwrap_or("development".to_string());
    if stage == "development" {
        dotenv().ok();
    }
    let port = u16::from_str_radix(&std::env::var("APP_PORT").unwrap(), 10).unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    clean_uploads_cron();

    HttpServer::new(move || {
        log::info!("Starting server on port {}", port);
        let api = web::scope("/api").service(pdf::routes());
        let cors = Cors::default()
            .allowed_origin("https://ipdf.lat")
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(Files::new("/uploads", "uploads").show_files_listing())
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

fn clean_uploads_cron() {
    thread::spawn(move || loop {
        let after_sec = env::var("CLEAN_AFTER_SEC")
            .unwrap_or("120".to_string())
            .parse::<u64>()
            .unwrap_or(120);

        delete_old_files("uploads", after_sec);
        delete_old_files("uploads/imgs", after_sec);
        log::info!("Cleaned old files after {} seconds", after_sec);
        thread::sleep(Duration::from_secs(after_sec));
    });
}
