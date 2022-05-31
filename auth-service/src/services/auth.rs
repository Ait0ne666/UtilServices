use crate::prelude::*;
use diesel::{ result::Error};
pub struct AuthService<'a> {
    pub repository:  &'a AppRepository<'a>
}



impl<'a>  AuthService<'a> {
    


    pub fn authenticate(&self, token: String) -> Result<AppWithLoggers, Error> {

        let maybe_app = self.repository.get_app(token);


        

        maybe_app
    }
}