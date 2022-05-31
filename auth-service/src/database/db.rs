// #[macro_use]
extern crate diesel;
extern crate dotenv;


use std::fmt::Debug;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection, Pool, PoolError};
use dotenv::dotenv;
use std::env;

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }



pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
     dotenv().ok();

     let database_url = env::var("DATABASE_URL")
         .expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
 }


#[allow(missing_debug_implementations)]
 pub struct DbConnPool {
     pub conn: PgPool
 }

 impl Debug for DbConnPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

 
 impl Default for DbConnPool {
     fn default() -> Self {
         DbConnPool {
             conn: establish_connection()
         }
     }
 }

