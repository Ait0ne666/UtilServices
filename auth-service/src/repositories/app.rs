use diesel::{prelude::*, result::Error};

use crate::prelude::*;



#[derive(Clone)]
pub struct AppRepository<'a> {
    pub connection: &'a PgPool,
}

impl<'a> AppRepository<'a> {
    pub fn new(connection: &'a PgPool) -> Self {
        AppRepository {
            connection: connection,
        }
    }


    pub fn get_connection(&self) -> PgPooledConnection{
            self.connection
            .get()
            .map_err(|e| {
                Error::NotFound
            }).unwrap()
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
            .get_result::<App>(&self.get_connection());

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
                    .get_result(&self.get_connection())
                    .expect("Error saving new post");

                Some(app)
            }
        }
    }

    pub fn delete_app(&self, t: String) -> usize {
        use crate::schema::apps::dsl::*;

        let num_deleted = diesel::delete(apps.filter(title.eq(t)))
            .execute(&self.get_connection())
            .expect("Error deleting app");

        num_deleted
    }

    pub fn update_app(&self, app: String, telegram: String) -> Result<App, Error> {
        use crate::schema::apps::dsl::*;

        let updated = diesel::update(apps.filter(title.eq(app)))
            .set(telegram_chat_id.eq(telegram))
            .get_result::<App>(&self.get_connection());

        updated
    }

    pub fn get_token(&self, app: String) -> Result<App, Error> {
        use crate::schema::apps::dsl::*;

        let item = apps
            .filter(title.eq(app))
            .get_result::<App>(&self.get_connection());

        item
    }


    pub fn get_app(&self, t: String) -> Result<AppWithLoggers, Error> {
        use crate::schema::apps::dsl::*;
        use crate::schema::loggers::dsl::*;

        let item = apps
            .filter(token.eq(t))
            .get_result::<App>(&self.get_connection());

        match item {
            Ok(a) => {
                let result = loggers.filter(app_id.eq(a.id)).load::<Logger>(&self.get_connection());

                match result {
                    Ok(r) => {
                        Ok(AppWithLoggers {
                            id: a.id,
                            loggers: r,
                            telegram_chat_id: a.telegram_chat_id,
                            title: a.title,
                            token: a.token
                        })
                    },
                    Err(e) => {
                        Err(e)
                    },
                }

            },
            Err(e) => {
                Err(e)
            },
        }
    }


}
