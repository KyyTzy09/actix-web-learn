use actix_web::Responder;

use crate::services::auth_service::{self, AuthService};


pub async fn register() -> impl Responder {    
    let auth_service = AuthService::new(&AuthService{  });
    auth_service.register().await
}
