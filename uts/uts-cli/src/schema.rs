
table! {
    apps (id) {
        id -> Int4,
        title -> Varchar,
        telegram_chat_id -> Nullable<Text>,
        token -> Nullable<Varchar>,
    }
}
