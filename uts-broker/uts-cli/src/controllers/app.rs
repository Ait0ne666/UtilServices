extern crate bcrypt;
use std::env;

use crate::{prelude::*, repositories::prelude::AppRepository};
use bcrypt::{DEFAULT_COST, hash,};
use diesel::{ result::Error};

pub fn add_app(title: String, telegram: Option<String>, app_repository: AppRepository) {
    let token = &create_token();



    let mut tg: Option<&str> = None;



    let mut temp: String = "".to_string();
    match telegram {
        Some(t) => {
            temp = t.replace("m", "-");
            tg = Some(&temp)
        },
        None => {}
    }

    let app = app_repository.create_app(&title, tg, token);


    match app {
        Some(_) => {
            println!("App successfully created! \nAPI-token: {}", token);

        },
        None => {
            println!("Couldn't create app");
        },
    }

}

pub fn delete_app(title: String, app_repository: AppRepository) {


    let res = app_repository.delete_app(title);

    if res == 0 {
        println!("No apps have been deleted")
    } else {
        println!("App successfully deleted")
    }
}

pub fn update_app(title: String, tg: String, app_repository: AppRepository) {


    let result = app_repository.update_app(title, tg);

    match result {
        Ok(updated) => {
            println!(
                "Telegram token for app '{}' successfully set to '{}'",
                updated.title, updated.telegram_chat_id.unwrap()
            );
        }
        Err(_) => {
            println!("Couldn't update app")
        }
    }
}





fn create_token() -> String {
    let h = env::var("HASH");


    match h {
        Ok(v) => {
            let hashed = hash(v, DEFAULT_COST);



            

            match hashed {
                Ok(s) => {
                    return s;
                }
                Err(e) => {
                    println!("{}", e);
                    return "".to_string()
                },
            }
        },
        Err(e) => {
            println!("{}", e);
            return "".to_string()
        },
    }



    
}




pub fn get_token_for_app(app: String, app_repository: AppRepository) -> Result<App, Error> {





    app_repository.get_token(app)


}


pub fn create_logger_for_app(app: String, logger: Option<String>, app_repository: AppRepository)  {


    match logger {
        Some(l) => {
            let maybe_type = LoggerTypes::from_str(l.clone());



            match maybe_type {
                Some(t) => {
                    let res = app_repository.create_logger_for_app(app.clone(), t);


                    match res {
                        Ok(created) => {
                            println!("{} logger created for app {}", l, app);
                        },
                        Err(e) => {
                            println!("Error creating logger: {}", e);
                        },
                    }
                },
                None => {
                    println!("Incorrect logger type");
                },
            }


        },
        None => {
            println!("No logger specified");
        },
    }


}



pub fn delete_logger_for_app(app: String, logger: Option<String>, app_repository: AppRepository)  {


    match logger {
        Some(l) => {
            let maybe_type = LoggerTypes::from_str(l.clone());



            match maybe_type {
                Some(t) => {
                    let res = app_repository.delete_logger_for_app(app.clone(), t);


                    match res {
                        Ok(deleted) => {
                            println!("{} logger successfully deleted for app {}", l, app);
                        },
                        Err(e) => {
                            println!("{}", e);
                        },
                    }
                },
                None => {
                    println!("Incorrect logger type")
                },
            }


        },
        None => {
            println!("No logger specified");
        },
    }


}




pub fn get_loggers_for_app(app: String,  app_repository: AppRepository) {

    let loggers = app_repository.get_loggers_for_app(app.clone());


    match loggers {
        Ok(ls) => {
            if ls.len() == 0 {
                println!("No loggers found for app {}", app);
            } else {
                println!("Loggers for app {}:", app);

                for log in ls {
                    println!("{}", log.logger_type);
                }

            }
        },
        Err(e) => {
            println!("There was an error getting loggers: {}", e);
        },
    }


}