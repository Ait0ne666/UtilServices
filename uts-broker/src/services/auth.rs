



use tonic::transport::Error;

use crate::{prelude::App, setup_grpc::{AuthRequest, auth_grpc_service_client::AuthGrpcServiceClient}};

pub struct AuthService {

}




impl AuthService {



    pub async fn authenticate(&self, token: &str) -> Option<App> {

        println!("Authenticating");



        

        let mut client = AuthGrpcServiceClient::connect("http://auth-service:50051").await;

        match client {
            Ok(ref mut c) => {
                let request = tonic::Request::new(AuthRequest {
                    token: token.to_string()
                });
        
                let response = c.authenticate(request).await;


                match response {
                    Ok(resp) => {
                        let data = resp.get_ref().clone();
                
                        
                
                
                        match data.app {
                            Some(a) => {
                                Some(App {
                                    id: a.id,
                                    telegram_chat_id: a.telegram_chat_id,
                                    title: a.title,
                                    token: Some(a.token),
                                    loggers: a.loggers
                                })
                            },
                            None => {
                                None
                            },
                        }

                    },
                    Err(e) => {
                        println!("error: {}", e);

                        None
                    },
                }
        
            },
            Err(e) => {
                println!("error: {}", e);
                None
            },
        }






    }
}


