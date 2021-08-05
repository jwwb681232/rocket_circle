use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use super::schema::users;
use crate::database::Db;
use diesel::prelude::*;
use super::schema::users::dsl::users as users_table;
use super::form::User as FormUser;
use rocket::form::Form;
use bcrypt::hash;
use crate::InternalResult;
use crate::error::ApiError::InternalError;
use crate::error::InternalError::Database;

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

    pub async fn create(db:Db,request:Form<FormUser>)->InternalResult<usize>{

        let req = request.clone();

        let res = db.run(move |conn|{

            diesel::insert_into(users_table)
                .values(
                    (
                        users::dsl::name.eq(req.name),
                        users::dsl::email.eq(req.email),
                        users::dsl::password.eq(hash(req.password,4).unwrap()),
                        users::dsl::remember_token.eq(""),
                        users::dsl::created_at.eq(Utc::now().naive_local()),
                        users::dsl::updated_at.eq(Utc::now().naive_local()),
                    )
                )
                .execute(conn).map_err(|e|InternalError(Database(e.to_string())))

        }).await?;

        println!("{:?}",res);
        Ok(res)
    }
}
