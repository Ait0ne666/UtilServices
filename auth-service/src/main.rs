#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod database;
mod repositories;
mod services;
mod grpc;



pub mod prelude {
    pub use crate::models::prelude::*;
    pub use crate::database::prelude::*;
    pub use crate::repositories::prelude::*;
    pub use crate::schema::*;
    pub use crate::services::*;
    pub use crate::grpc::prelude::*;
}


use std::env;

use repositories::prelude::AppRepository;
use services::prelude::AuthService;
use setup_grpc::auth_grpc_service_server::AuthGrpcServiceServer;

use crate::prelude::*;
use tonic::{transport::Server, Request, Response, Status};



pub mod setup_grpc {
    tonic::include_proto!("auth"); // The string specified here must match the proto package name
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env();


    let conn = establish_connection();




    let addr = "0.0.0.0:50051".parse()?;
    let grpc = GRPCService::new(DbConnPool { conn: conn });

    Server::builder()
        .add_service(AuthGrpcServiceServer::new(grpc))
        .serve(addr)
        .await?;




    Ok(())
}





fn load_env() {
    let mode = env::var("APP_MODE");


    match mode  {
        Ok(m) => {
            if m != "PRODUCTION" {
                dotenv::dotenv().expect("No .env file found");
            }
        },
        Err(_) => {
            dotenv::dotenv().expect("No .env file found");
        },
    }
}