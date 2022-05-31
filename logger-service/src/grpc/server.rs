
use std::{rc::Rc, sync::Arc};

use crate::{
    
    setup_grpc::{logger_grpc_service_server::LoggerGrpcService, LoggerRequest, LoggerResponse}, prelude::{ Logger, Severity},
};
use tonic::{Request, Response, Status};


pub struct GRPCService {
    logger: Arc<Logger>
}


impl GRPCService {
    pub fn new(logger: Arc<Logger>) -> Self {
        GRPCService { logger: logger }
    }
}

#[tonic::async_trait]
impl LoggerGrpcService for GRPCService {
    async fn log(
        &self,
        request: Request<LoggerRequest>,
    ) -> Result<Response<LoggerResponse>, Status> {
        println!("Got a request: {:?}", request);

       
        let data = request.get_ref();
        
        let res = self.logger.log(&data.message, Severity::from(data.severity.clone()), data.app.clone().unwrap()).await;
        
        
        let reply = LoggerResponse {
            error: res
        };
        


        Ok(Response::new(reply))


    }
}
