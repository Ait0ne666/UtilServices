#[macro_use]
extern crate derive_more;

mod models;
mod handlers;
mod middleware;
mod services;
mod errors;
use actix_web::{error, web, App, HttpResponse, HttpServer};
use setup_grpc::auth_grpc_service_client::AuthGrpcServiceClient;

pub mod prelude {
    pub use crate::handlers::prelude::*;
    pub use crate::middleware::prelude::*;
    pub use crate::models::prelude::*;
    pub use crate::services::prelude::*;
    pub use crate::services::prelude::*;
}

use crate::prelude::*;

const WEB_PORT: u16 = 3000;

use tonic::{transport::Server, Request, Response, Status};



pub mod setup_grpc {
    tonic::include_proto!("auth"); 
}


pub mod logger_grpc {
    tonic::include_proto!("logger");
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    


    println!("Starting server");
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                let e = err.to_string();

                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(HandlerResponse {
                        error: true,
                        message: e,
                    }),
                )
                .into()
            });
        let auth_service = AuthService {

        };

        App::new()
            .app_data(json_config)
            .service(web::scope("/api")
            .wrap(AuthMiddlewareFactory::new(auth_service))
            .route("/handle", web::post().to(handle)))
    })
    
    .bind(("0.0.0.0", WEB_PORT))?
    .run()
    
    
    .await


    
}
