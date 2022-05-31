use std::{
    future::{ready, Ready},
  
    rc::Rc,
};

use actix_web::{
    dev::{ Service, ServiceRequest, ServiceResponse, Transform},
    error, Error, HttpMessage, HttpResponse, FromRequest, HttpRequest,
};
use futures_util::future::LocalBoxFuture;

use crate::{prelude::{App, AuthService, HandlerResponse}, errors::prelude::CustomError};

pub struct AuthMiddleware<S> {
    auth_service: Rc<AuthService>,
    service: Rc<S>,
}

pub type AuthResult = Rc<App>;

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        

        let contains = req.headers().contains_key("API_KEY");

        if !contains {
            return Box::pin(async move {
                Err(error::InternalError::from_response(
                    "",
                    HttpResponse::Forbidden().json(HandlerResponse {
                        error: true,
                        message: "Unauthorized".to_string(),
                    }),
                )
                .into())
            });
        }

        let token = req
            .headers()
            .get("API_KEY")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let auth_service = self.auth_service.clone();
        let srv = self.service.clone();

        Box::pin(async move {
            let maybe_app = auth_service.authenticate(&token).await;

            match maybe_app {
                Some(app) => {
                    req.extensions_mut().insert(app);
                }
                None => {
                    return Err(CustomError::Auth.into())
                }
            }

            let res = srv.call(req).await?;

            Ok(res)
        })
    }
}

pub struct AuthMiddlewareFactory {
    auth_service: Rc<AuthService>,
}

impl AuthMiddlewareFactory {
    pub fn new(auth_service: AuthService) -> Self {
        AuthMiddlewareFactory {
            auth_service: Rc::new(auth_service),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            auth_service: self.auth_service.clone(),
            service: Rc::new(service),
        }))
    }
}






pub struct Authenticated(App);

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;




    fn from_request(req: &HttpRequest,
            _payload: &mut actix_web::dev::Payload) -> Self::Future {

        let value = req.extensions().get::<App>().cloned();
        let result = match value {
            Some(v) => Ok(Authenticated(v)),
            None => Err(CustomError::Auth.into()),
        };
        ready(result)
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}

impl std::ops::Deref for Authenticated {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}