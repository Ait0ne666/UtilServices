extern crate bcrypt;
use std::env;

use crate::{prelude::*, repositories::prelude::AppRepository};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::{ result::Error};

pub fn add_app(title: String, telegram: Option<String>) {
    let token = &create_token();

    let conn = establish_connection();

    let app_repository = AppRepository::new(&conn);

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

pub fn delete_app(title: String) {
    let conn = establish_connection();

    let app_repository = AppRepository::new(&conn);

    let res = app_repository.delete_app(title);

    if res == 0 {
        println!("No apps have been deleted")
    } else {
        println!("App successfully deleted")
    }
}

pub fn update_app(title: String, tg: String) {
    let conn = establish_connection();

    let app_repository = AppRepository::new(&conn);

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




pub fn get_token_for_app(app: String) -> Result<App, Error> {

    let conn = establish_connection();

    let app_repository = AppRepository::new(&conn);



    app_repository.get_token(app)


}