use crate::{logger_grpc::{logger_grpc_service_client::LoggerGrpcServiceClient, LoggerRequest, App}, models::prelude::{LogData}};




pub async fn send_log(data: LogData, app: App) -> bool {
    let mut client = LoggerGrpcServiceClient::connect("http://logger-service:50051").await;


    match client {
        Ok(ref mut c) => {
            let request = tonic::Request::new(LoggerRequest {
                app: Some(App {
                    telegram_chat_id: app.telegram_chat_id,
                    id: app.id,
                    title: app.title,
                    token: app.token,
                    loggers: app.loggers
                }),
                message: data.message,
                severity: data.severity.into()
            });
    
            let response = c.log(request).await;


            match response {
                Ok(resp) => {
                    let data = resp.get_ref().clone();
            
                    
            
                    data.error


                },
                Err(e) => {
                    println!("error: {}", e);

                    false
                },
            }
    
        },
        Err(e) => {
            println!("error: {}", e);
            false
        },
    }
    
}