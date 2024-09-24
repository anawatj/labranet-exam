#[macro_use]
extern crate rocket;

use labranet_auth::db::db::connect;
use labranet_auth::db::db::MongoDB;
use labranet_auth::handlers::auth::current_user;
use labranet_auth::handlers::auth::login;
use labranet_auth::handlers::users::sign_up;
use labranet_auth::repositories::users::UserRepo;
use labranet_auth::repositories::users::UserRepoTrait;
use labranet_auth::usecases::users::UserUseCase;
use labranet_auth::usecases::users::UserUseCaseTrait;
use labranet_common::response::ResponseError;
use labranet_common::response::ResponseErrorBody;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::shield::Allow;
use rocket::shield::Permission;
use rocket::shield::Shield;
use rocket_cors::{AllowedOrigins, CorsOptions};




#[launch]
async fn rocket() -> _ {
    let url = uri!("https://labranet.exam.dev");

    let permission = Permission::default()
    .allow(rocket::shield::Feature::Gyroscope,[Allow::This,Allow::Origin(url)]);
    
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    let database = connect().await.unwrap();
    let mongo = MongoDB::new(database);
    let user_repo : Box<dyn UserRepoTrait>=Box::new(UserRepo::new(mongo));
    let user_usecase: Box<dyn UserUseCaseTrait> = Box::new(UserUseCase::new(user_repo));
    rocket::build()
        .attach(Shield::default().enable(permission))
        .manage(user_usecase)
        .manage(cors.to_cors())
        .mount(
            "/api/v1",
            routes![
                sign_up,
                login,
                current_user
                
            ],
        )
        .register("/", 
        catchers![unauthorized,not_found,internal_sever_error,bad_gateway]
    )
       
}

#[catch(401)]
pub fn unauthorized() -> Json<ResponseError<String>> {
    let response = ResponseError{
        error:ResponseErrorBody::<String>::Error("Unauthorize".to_string())
    };
    Json(response)
}

#[catch(404)]
pub fn not_found() -> Json<ResponseError<String>> {
    let response = ResponseError{
        error:ResponseErrorBody::<String>::Error("Not Found".to_string())
    };
    Json(response)
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ResponseError<String>> {
    let response = ResponseError{
        error:ResponseErrorBody::<String>::Error("UNKNOWN Json".to_string())
    };
    Json(response)
}

#[catch(502)]
pub fn bad_gateway() -> Json<ResponseError<String>> {
    let response = ResponseError{
        error:ResponseErrorBody::<String>::Error("Bad Gateway".to_string())
    };
    Json(response)
}

