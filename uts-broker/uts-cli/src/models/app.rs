
use crate::{schema::{apps}, prelude::Logger};



#[derive(Queryable, Identifiable,  PartialEq, Debug)]
#[table_name="apps"]
pub struct App {
    pub id: i32,
    pub title: String,
    pub telegram_chat_id: Option<String>,
    pub token: Option<String>,
    
}



#[derive(Insertable)]
#[table_name="apps"]
pub struct NewApp<'a> {
    pub title: &'a str,
    pub telegram_chat_id: Option<&'a str>,
    pub token: &'a str
}



