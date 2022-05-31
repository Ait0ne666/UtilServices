
use actix_web::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use actix_web::HttpRequest;
use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResponse {
    pub error: bool,
    pub message: String,
}




impl Responder for HandlerResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}