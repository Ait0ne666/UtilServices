use diesel::{prelude::*, result::Error};

use crate::prelude::*;


pub struct AppRepository<'a> {
    pub connection: &'a PgConnection,
}

impl<'a> AppRepository<'a> {
    pub fn new(connection: &'a PgConnection) -> Self {
        AppRepository {
            connection: connection,
        }
    }

    pub fn create_app(
        &self,
        app: &'a str,
        telegram: Option<&'a str>,
        t: &'a str,
    ) -> Option<App> {
        use crate::schema::apps::dsl::*;

        let exist = apps
            .filter(title.eq(app))
            .get_result::<App>(self.connection);

        match exist {
            Ok(_) => {
                return None;
            }
            Err(_) => {
                let new_app = NewApp {
                    title: app,
                    telegram_chat_id: telegram,
                    token: t,
                };

                let app = diesel::insert_into(apps)
                    .values(&new_app)
                    .get_result(self.connection)
                    .expect("Error saving new post");

                Some(app)
            }
        }
    }

    pub fn delete_app(&self, t: String) -> usize {
        use crate::schema::apps::dsl::*;

        let num_deleted = diesel::delete(apps.filter(title.eq(t)))
            .execute(self.connection)
            .expect("Error deleting app");

        num_deleted
    }

    pub fn update_app(&self, app: String, telegram: String) -> Result<App, Error> {
        use crate::schema::apps::dsl::*;

        let updated = diesel::update(apps.filter(title.eq(app)))
            .set(telegram_chat_id.eq(telegram))
            .get_result::<App>(self.connection);

        updated
    }

    pub fn get_token(&self, app: String) -> Result<App, Error> {
        use crate::schema::apps::dsl::*;

        let item = apps
            .filter(title.eq(app))
            .get_result::<App>(self.connection);

        item
    }


    pub fn create_logger_for_app(&self, app: String, logger: LoggerTypes) -> Result<Logger, Error> {

        use crate::schema::apps::dsl::*;
        use crate::schema::loggers::dsl::*;

        let full_app = apps
        .filter(title.eq(app))
        .get_result::<App>(self.connection);


        let logger_str: String = logger.into();

        match full_app {
            Ok(ref fa) => {
                let log = loggers.filter(app_id.eq(fa.id)).filter(logger_type.eq(&logger_str)).get_result::<Logger>(self.connection);



                match log {
                    Ok(logger) => {
                        Ok(logger)
                    },
                    Err(e) => {
                        let new_logger = NewLogger {
                            app_id: fa.id,
                            logger_type: &logger_str
                        };

                        let inserted = diesel::insert_into(loggers)
                        .values(&new_logger)
                        .get_result::<Logger>(self.connection);

                        inserted
                    },
                }



                
            },
            Err(e) => {
                Err(e)
            },
        }


    }



    pub fn delete_logger_for_app(&self, app: String, logger: LoggerTypes) -> Result<bool, String> {
        use crate::schema::apps::dsl::*;
        use crate::schema::loggers::dsl::*;


        let full_app = apps
        .filter(title.eq(app))
        .get_result::<App>(self.connection);


        let logger_str: String = logger.into();


        match full_app {
            Ok(fa) => {


                let result = diesel::delete(loggers.filter(app_id.eq(fa.id)).filter(logger_type.eq(&logger_str))).execute(self.connection);



                match result {
                    Ok(count) => {
                        if count > 0 {
                            Ok(true)
                        } else {
                            Err("Nothing to delete".to_string())
                        }
                    },
                    Err(_) => {
                        Err("Nothing to delete".to_string())
                    },
                }


                
            },
            Err(_) => {
                return Err("App not found".to_string())
            },
        }
    }


    pub fn get_loggers_for_app(&self, app: String) -> Result<Vec<Logger>, Error> {


        use crate::schema::apps::dsl::*;
        use crate::schema::loggers::dsl::*;


        let full_app = apps
        .filter(title.eq(app))
        .get_result::<App>(self.connection);



        match full_app {
            Ok(fa) => {
                let result = loggers.filter(app_id.eq(fa.id)).load::<Logger>(self.connection);



                result
            },
            Err(e) => {
                Err(e)
            },
        }
    } 

}
