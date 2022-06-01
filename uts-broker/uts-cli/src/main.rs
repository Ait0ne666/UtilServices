#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate dotenv;

use diesel_migrations::embed_migrations;



embed_migrations!("./migrations/");


mod database;
mod models;
mod repositories;
mod schema;
mod controllers;

pub mod prelude {
    pub use crate::database::prelude::*;
    pub use crate::models::prelude::*;
    pub use crate::schema::*;
    pub use crate::controllers::prelude::*;
}

use std::env;

use clap::{ArgEnum, Parser};
use prelude::{ add_app, delete_app, update_app, get_token_for_app, establish_connection, create_logger_for_app, delete_logger_for_app, get_loggers_for_app};
use repositories::prelude::AppRepository;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Action {
    Add,
    Delete,
    Update,
    Token,
    AddLogger,
    DeleteLogger,
    Loggers
}

#[derive(Parser)]
#[clap(name = "uts-cli")]
#[clap(author = "Ait0ne666 <bonafide112358@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "cli for adding and deleting apps from util microservices", long_about = None)]
struct Cli {
    #[clap(short = 'a', long)]
    app: String,

    #[clap(arg_enum, short = 'c', long)]
    action: Action,

    #[clap(short = 't', long)]
    telegram: Option<String>,

    #[clap(short = 'l', long)]
    logger: Option<String>,
}

fn main() {
    load_env();

    let cli = Cli::parse();

    let conn = establish_connection();

    let res = embedded_migrations::run(&conn);


    match res {
        Ok(_) => {
            println!("Migrations run successfully");
        },
        Err(e) => {
            println!("Error running migrations:\n{}", e);
        },
    }

    let app_repository = AppRepository::new(&conn);

    match cli.action {
        Action::Add => {
            println!("Adding {}", cli.app);

            add_app(cli.app, cli.telegram, app_repository);
            return;
        }
        Action::Delete => {
            println!("Deleting {}", cli.app);
            delete_app(cli.app, app_repository);
            return;
        }
        Action::Update => {
            match cli.telegram {
                Some(t) => {
                    update_app(cli.app, t.replace("m", "-"), app_repository)
                }
                None => {
                    println!("New telegram token not provided!!!");
                    return;
                }
            }

            return;
        }
        Action::Token => {
            match get_token_for_app(cli.app, app_repository) {
                Ok(app) => {

                    match app.token {
                        Some(ref t) => {
                            println!("Token for an app {} is: {}", app.title, t);
                        }
                        None => {
                            println!("There is no token for an app {}", app.title)
                        }
                    }
                    return;
                },
                Err(_) => {
                    println!("App not found");
                    return;
                },
            }
        },
        Action::AddLogger => {
            create_logger_for_app(cli.app, cli.logger, app_repository)
        },
        Action::DeleteLogger => {
            delete_logger_for_app(cli.app, cli.logger, app_repository)
        },
        Action::Loggers => {
            get_loggers_for_app(cli.app, app_repository)
        },
    }
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