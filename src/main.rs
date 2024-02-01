use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct TempInput {
    unit: String,
    temperature: f32,
}

#[post("/convert")]
async fn convert(temp_input: web::Form<TempInput>) -> impl Responder {
    let converted_temp = if temp_input.unit == "f" {
        (temp_input.temperature - 32.0) * 5.0 / 9.0
    } else {
        (temp_input.temperature * 9.0 / 5.0) + 32.0
    };

    let response_msg = format!("Converted temperature: {}", converted_temp as i32);
    HttpResponse::Ok().body(response_msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new().wrap(cors).service(convert)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
