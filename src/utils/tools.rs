use actix_web::HttpResponse;

pub fn make_response(status: &str, data: serde_json::Value) -> HttpResponse {
    let mut response = serde_json::Map::new();
    response.insert(
        "status".to_string(),
        serde_json::Value::String(status.to_string()),
    );
    response.insert("data".to_string(), data);

    match status {
        "SUCCED" => HttpResponse::Ok().json(response),
        "FAILED" => HttpResponse::BadRequest().json(response),
        _ => HttpResponse::InternalServerError().json(response),
    }
}
