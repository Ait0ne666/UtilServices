// use diesel::{Queryable, Identifiable, Associations};
use crate::schema::{loggers};
use crate::models::app::App;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Logger {
    pub id: i32,
    pub app_id: i32,
    pub logger_type: String 
}


#[derive(Insertable)]
#[table_name="loggers"]
pub struct NewLogger<'a> {
    pub app_id: i32,
    pub logger_type: &'a str 
}





pub enum LoggerTypes {
    File,
    Telegram
}


impl LoggerTypes {
    pub fn from_str(s: String) -> Option<Self> {
        match s.as_str() {
            "file" => {
                Some(LoggerTypes::File)
            } 
            "telegram" => {
                Some(LoggerTypes::Telegram)
            }
            _ => {
                None
            }
        }
    }
}




impl Into<String> for LoggerTypes {
    fn into(self) -> String {
        match self {
            LoggerTypes::File => {
                "file".to_string()
            },
            LoggerTypes::Telegram => {
                "telegram".to_string()
            },
        }
    }
}