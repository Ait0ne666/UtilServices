
#[derive(Clone)]
pub struct App {
    pub id: i32,
    pub title: String,
    pub telegram_chat_id: Option<String>,
    pub token: Option<String>,
    pub loggers: Vec<String>
}