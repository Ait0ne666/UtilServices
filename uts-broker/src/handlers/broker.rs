use actix_web::{web, Either, HttpResponse};

use crate::{prelude::{HandlerRequest, HandlerResponse, Authenticated}, services::prelude::send_log, logger_grpc::App};



pub async fn handle(body: web::Json<HandlerRequest>, app: Authenticated) -> Either<HttpResponse, HandlerResponse> {


    let  data = body.0;


    

    match data.action {
        crate::prelude::Actions::LOG => {
            match data.log {
                Some(log) => {

                    let res = send_log(log, App {
                        id: app.id,
                        token: app.token.clone().unwrap(), 
                        telegram_chat_id: app.telegram_chat_id.clone(),
                        title: app.title.clone(),
                        loggers: app.loggers.clone()
                    }).await;


                    if !res {
                        return Either::Right(HandlerResponse {
                            error: false,
                            message: "Successfully logged".to_string() + &app.title
                        })

                    } else {
                        println!("There was an error logging");
                        return Either::Right(HandlerResponse {
                            error: false,
                            message: "Error logging".to_string() + &app.title
                        })
                    }
                    
                },
                None => {
                    return Either::Left(HttpResponse::BadRequest().json(HandlerResponse {
                        error: true,
                        message: "Log field should be specified".to_string()
                    }))
                },
            }
        },
    }


}