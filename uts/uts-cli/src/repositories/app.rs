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
}
