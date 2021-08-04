use chrono::NaiveDateTime;
use serde::Serialize;
use super::schema::users;
use crate::database::Db;
use diesel::prelude::*;
use super::schema::users::dsl::users as users_table;

#[derive(Serialize, Debug, AsChangeset, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub remember_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn list(db: Db) -> Option<Vec<User>> {
        let results: Option<Vec<User>> = db.run(move |conn| {
            users_table.limit(15).load::<User>(conn)
        }).await.ok();

        //println!("{:#?}",results);

        results
    }
}
