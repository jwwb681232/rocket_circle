use crate::bootstrap::database::Db;
use rocket::form::Form;
use rocket::serde::json::Json;
use serde::Serialize;
use diesel::prelude::*;
use bcrypt::{verify};

#[derive(Debug, FromForm, Clone)]
pub struct LoginRequest{
    email:String,
    password:String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    code: u16,
    data: Option<String>,
    message: String,
}


#[post("/login",data="<request>")]
pub async fn login(db:Db,request:Form<LoginRequest>)->Json<LoginResponse>{

    let req = request.clone();

    let password:Result<String,diesel::result::Error> = db.run(move |conn| {
        crate::schema::users::dsl::users
            .filter(crate::schema::users::email.eq(req.email))
            .select(crate::schema::users::password)
            .first::<String>(conn)

    }).await;

    if verify(&request.password,password.unwrap().as_str()).unwrap() {
        Json(LoginResponse{
            code: 200,
            data: Some(1.to_string()),
            message: "".to_string()
        })
    }else{
        Json(LoginResponse{
            code: 403,
            data: Some(1.to_string()),
            message: "".to_string()
        })
    }


}
