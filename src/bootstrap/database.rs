use rocket_sync_db_pools::diesel;

#[database("diesel")]
pub struct Db(diesel::MysqlConnection);
