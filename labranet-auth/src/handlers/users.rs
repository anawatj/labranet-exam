use rocket::State;
use rocket::serde::json::Json;
use rocket::{
    post,
    response::status::{Created, Custom},
};
use crate::{
    models::users::UserModel,
    usecases::users::UserUseCaseTrait,
};

#[post("/users", format = "application/json", data = "<user>")]
pub async fn sign_up(user_use_case: &State<Box<dyn UserUseCaseTrait>>,user: Json<UserModel>) -> Result<Created<String>, Custom<String>> {
    
    let result = user_use_case.sign_up(user.into_inner()).await;
    result
}


