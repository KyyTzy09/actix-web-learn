use serde::Serialize;

#[derive(Debug, Serialize)]
struct CustomResponse {
    message: String,
    status: i32,
    data: Option<String>,
}

pub fn SuccessResponse(
    message: String,
    status: i32,
    data: Option<String>,
) -> impl actix_web::Responder {
    let response = CustomResponse {
        message,
        status,
        data,
    };

    actix_web::HttpResponse::Ok().json(response)
}
