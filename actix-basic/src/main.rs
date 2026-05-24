use::actix_web::{App, HttpServer};

mod routes;
mod models;
mod handlers;
mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server running at Port 8080"); 
    HttpServer::new(|| {
        App::new()
        .configure(routes::route_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

