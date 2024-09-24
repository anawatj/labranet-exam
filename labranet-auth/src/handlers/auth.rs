use labranet_common::{jwt::JWT, response:: ResponseError};
use rocket::{get, post, response::status::Custom, serde::json::Json, State};

use crate::{models::login::LoginModel, usecases::users::UserUseCaseTrait};

#[post("/auth/login", format = "application/json", data = "<login>")]
pub async  fn login(user_use_case: &State<Box<dyn UserUseCaseTrait>>,login:Json<LoginModel>)->Result<String,Custom<String>>{
 
    let result = user_use_case.login(login.into_inner()).await;
    result
}
#[get("/auth/current-user")]
pub async fn current_user(user_use_case: &State<Box<dyn UserUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,)->Result<String,Custom<String>>{
    let result = user_use_case.get_current_user(key).await;
    result
}