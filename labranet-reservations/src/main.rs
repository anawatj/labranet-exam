#[macro_use]
extern crate rocket;


use labranet_common::response::ResponseError;
use labranet_common::response::ResponseErrorBody;
use labranet_reservations::db::db::connect;
use labranet_reservations::db::db::MongoDB;
use labranet_reservations::handlers::reservations::delete_reservation;
use labranet_reservations::handlers::reservations::fetch_all_reservation;
use labranet_reservations::handlers::reservations::fetch_one_reservation;
use labranet_reservations::handlers::reservations::new_reservation;
use labranet_reservations::handlers::reservations::update_reservation;
use labranet_reservations::repositories::reservations::ReservationRepo;
use labranet_reservations::repositories::reservations::ReservationRepoTrait;
use labranet_reservations::usecases::reservations::ReservationUseCase;
use labranet_reservations::usecases::reservations::ReservationUseCaseTrait;
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
    let reservation_repo : Box<dyn ReservationRepoTrait>=Box::new(ReservationRepo::new(mongo));
    let reservation_use_case: Box<dyn ReservationUseCaseTrait> = Box::new(ReservationUseCase::new(reservation_repo));
    rocket::build()
        .attach(Shield::default().enable(permission))
        .manage(reservation_use_case)
        .manage(cors.to_cors())
        .mount(
            "/api/v1",
            routes![
                new_reservation,
                fetch_all_reservation,
                fetch_one_reservation,
                update_reservation,
                delete_reservation,
                
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
