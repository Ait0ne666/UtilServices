mod grpc;
mod telegram;
mod services;
mod file_logger;

extern crate chrono;


pub mod prelude {
    pub use crate::grpc::prelude::*;
    pub use crate::telegram::prelude::*;
    pub use crate::services::prelude::*;
    pub use crate::file_logger::prelude::*;
}


use std::{env, sync::Arc};

use crate::{prelude::*, setup_grpc::App};

use flexi_logger::{FileSpec, Criterion, Age, Naming};
use setup_grpc::logger_grpc_service_server::LoggerGrpcServiceServer;
use tonic::{transport::Server};



pub mod setup_grpc {
    tonic::include_proto!("logger"); // The string specified here must match the proto package name
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env();



    let addr = "0.0.0.0:50051".parse()?;
    let  handle =    FileLogger::create_handle();

    


    let  fl = FileLogger::new(Arc::new(handle));
    let http_client = reqwest::Client::builder().build().unwrap();


    

    let  logger:Logger = Logger {
        file_logger: Arc::new(fl),
        client: http_client
    };


    let grpc = GRPCService::new(Arc::new(logger.clone()));
    



    Server::builder()
        .add_service(LoggerGrpcServiceServer::new(grpc))
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