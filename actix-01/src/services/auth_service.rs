use actix_web::{HttpResponse};

pub struct AuthService {}

impl AuthService {
    pub fn new(&self) -> Self {
        Self {}
    }

    pub async fn register(&self) -> HttpResponse {
        HttpResponse::Ok().body("Registering")
    }
}
