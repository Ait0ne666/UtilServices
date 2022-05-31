

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::{Display, Error};

use crate::prelude::HandlerResponse;

#[derive(Debug, Display, Error)]
pub enum CustomError {
    
    Auth,
}




impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {

            CustomError::Auth => StatusCode::FORBIDDEN
        } 
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match *self {
            CustomError::Auth => {
                HttpResponse::Forbidden().json(HandlerResponse {
                    error: true,
                    message: "Unauthorized".to_string(),
                })
            }
        }
    }


}
