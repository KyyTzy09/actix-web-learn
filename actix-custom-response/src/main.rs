use actix_web::{HttpResponse, Responder, web};
use actix_web::{App, HttpServer};
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;

mod helpers;

// Response type
#[derive(Debug, Serialize)]
struct CustomResponse {
    message: String,
    status: u16,
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    let response = CustomResponse {
        message: "Hello, World!".to_string(),
        status: 200,
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::get("/custom")]
async fn custom_response() -> impl Responder {
    helpers::response::SuccessResponse("This is a custom response".to_string(), 200, None)
}

#[actix_web::post("login")]
async fn login() -> impl Responder {
    helpers::response::SuccessResponse("Login successful".to_string(), 200, None)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "mysql://root:@localhost:3306/actix_db";
    let pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect(db_url)
    .await
    .expect("Failed to connect to the database");

    println!("Server running at Port 8080");
    HttpServer::new(move || App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(index)
    .service(custom_response))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
