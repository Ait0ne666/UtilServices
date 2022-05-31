use crate::prelude::*;
use crate::setup_grpc::App;
use crate::{
    prelude::prelude::AuthService,
    setup_grpc::{auth_grpc_service_server::AuthGrpcService, AuthRequest, AuthResponse},
};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct GRPCService {
    conn: DbConnPool
}

impl GRPCService {
    pub fn new(conn: DbConnPool) -> Self {
        GRPCService { conn: conn }
    }
}




#[tonic::async_trait]
impl AuthGrpcService for GRPCService {
    async fn authenticate(
        &self,
        request: Request<AuthRequest>,
        
    ) -> Result<Response<AuthResponse>, Status> {
        println!("Got a request: {:?}", request);

        let conn = self.conn.conn.clone();

        let repository = AppRepository::new(&conn);

        let auth_service = AuthService {
            repository: &repository,
        };

        let result = auth_service.authenticate(request.get_ref().clone().token);

        match result {
            Ok(app) => {
                let reply = AuthResponse {
                    authenticated: true,
                    app: Some(App {
                        id: app.id,
                        telegram_chat_id: app.telegram_chat_id,
                        title: app.title,
                        token: app.token.unwrap(),
                        loggers: app.loggers.into_iter().map(|l| l.logger_type).collect()
                    }),
                };
        
                Ok(Response::new(reply))
            },
            Err(_) => {
                let reply = AuthResponse {
                    authenticated: false,
                    app: None,
                };
        
                Ok(Response::new(reply))
            },
        }


    }
}
