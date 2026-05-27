use actix_web::{App, FromRequest, HttpResponse, HttpServer, Responder, get, web};
use sqlx::mysql::MySqlPoolOptions;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login berhasil! Token: token_bearer")
}

#[actix_web::get("/profile")]
async fn get_profile(auth: AuthenticatedUser) -> impl Responder {
    HttpResponse::Ok().body(format!("Halo User dengan ID: {}", auth.user_id))
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    user_id: i32,
}

impl FromRequest for AuthenticatedUser{
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(v) => v.to_str().unwrap_or(""),
            None => return std::future::ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
        };

        let token = match auth_header.strip_prefix("Bearer ") {
            Some(t) => t,
            None => return std::future::ready(Err(actix_web::error::ErrorUnauthorized("Format token harus Bearer <token>")))
        };

        match token {
            "token_bearer" => std::future::ready(Ok(AuthenticatedUser { user_id: 99 })),
            _ => std::future::ready(Err(actix_web::error::ErrorUnauthorized("Token palsu atau sudah kedaluwarsa"))),
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "mysql://root:@localhost:3306/actix_web";
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to connect to the database");

    println!("Server running at Port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(login)
            .service(get_profile)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
