use actix_web::web::{self, post, scope};

use crate::handlers;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/ping", web::get().to(|| async { "pong" }))
            .service(
                web::scope("/auth")
                    .route("register", web::post().to(handlers::auth_handler::register)),
            )
    );
}