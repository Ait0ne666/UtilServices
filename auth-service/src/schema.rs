table! {
    apps (id) {
        id -> Int4,
        title -> Varchar,
        telegram_chat_id -> Nullable<Text>,
        token -> Nullable<Varchar>,
    }
}

table! {
    loggers (id) {
        id -> Int4,
        app_id -> Int4,
        logger_type -> Varchar,
    }
}

joinable!(loggers -> apps (app_id));

allow_tables_to_appear_in_same_query!(
    apps,
    loggers,
);
