#[macro_use]
extern crate diesel;
extern crate dotenv;

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
use prelude::{ add_app, delete_app, update_app, get_token_for_app};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Action {
    Add,
    Delete,
    Update,
    Token
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
}

fn main() {
    load_env();

    let cli = Cli::parse();

    match cli.action {
        Action::Add => {
            println!("Adding {}", cli.app);

            add_app(cli.app, cli.telegram);
            return;
        }
        Action::Delete => {
            println!("Deleting {}", cli.app);
            delete_app(cli.app);
            return;
        }
        Action::Update => {
            match cli.telegram {
                Some(t) => {
                    update_app(cli.app, t.replace("m", "-"))
                }
                None => {
                    println!("New telegram token not provided!!!");
                    return;
                }
            }

            return;
        }
        Action::Token => {
            match get_token_for_app(cli.app) {
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