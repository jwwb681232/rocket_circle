use crate::WebResult;
use rocket::serde::json::Json;
use crate::bootstrap::database::Db;

use crate::diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::models::user::UserIndex;

#[get("/")]
pub async fn index(db: Db) -> WebResult<Json<Vec<UserIndex>>> {
    let results:Vec<UserIndex> = db.run(move |conn| {

        users.limit(15).load::<UserIndex>(conn)

    }).await?;

    results.iter().map(|item|{
        println!("{}",item.created_at)
    });

    Ok(Json(results))






    // let ids: Vec<Option<i32>> = vec![Some(1)];
    //
    // Ok(Json(ids))
}
