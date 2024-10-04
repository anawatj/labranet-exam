#[macro_use]
extern crate rocket;


use std::env;

use labranet_common::events::Listener;
use labranet_common::response::ResponseError;
use labranet_common::response::ResponseErrorBody;
use labranet_rooms::db::db::connect;
use labranet_rooms::db::db::MongoDB;
use labranet_rooms::events::events::ReservationApproveListen;
use labranet_rooms::handlers::rooms::delete_building;
use labranet_rooms::handlers::rooms::delete_floor;
use labranet_rooms::handlers::rooms::delete_room;
use labranet_rooms::handlers::rooms::fetch_all_building;
use labranet_rooms::handlers::rooms::fetch_all_floor;
use labranet_rooms::handlers::rooms::fetch_all_room;
use labranet_rooms::handlers::rooms::fetch_one_building;
use labranet_rooms::handlers::rooms::fetch_one_floor;
use labranet_rooms::handlers::rooms::fetch_one_room;
use labranet_rooms::handlers::rooms::new_building;
use labranet_rooms::handlers::rooms::new_floor;
use labranet_rooms::handlers::rooms::new_room;
use labranet_rooms::handlers::rooms::update_building;
use labranet_rooms::handlers::rooms::update_floor;
use labranet_rooms::handlers::rooms::update_room;
use labranet_rooms::repositories::buildings::BuildingRepo;
use labranet_rooms::repositories::buildings::BuildingRepoTrait;
use labranet_rooms::repositories::floors::FloorRepo;
use labranet_rooms::repositories::floors::FloorRepoTrait;
use labranet_rooms::repositories::rooms::RoomRepo;
use labranet_rooms::repositories::rooms::RoomRepoTrait;
use labranet_rooms::usecases::rooms::RoomUseCase;
use labranet_rooms::usecases::rooms::RoomUseCaseTrait;
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
    let nats = nats::connect(env::var("NATS_URL").expect("nat url not config")).unwrap();
    let mongo = MongoDB::new(database);
    let building_repo:Box<dyn BuildingRepoTrait> = Box::new(BuildingRepo::new(mongo.clone()));
    let floor_repo:Box<dyn FloorRepoTrait> = Box::new(FloorRepo::new(mongo.clone()));
    let room_repo:Box<dyn RoomRepoTrait> = Box::new(RoomRepo::new(mongo.clone()));
    let room_repo_listen:Box<dyn RoomRepoTrait> = Box::new(RoomRepo::new(mongo.clone()));
    let room_use_case :Box<dyn RoomUseCaseTrait> = Box::new(RoomUseCase::new(building_repo, floor_repo, room_repo));
    let reservation_approve_listener :Box<dyn ReservationApproveListen> = Box::new(Listener::new(nats,"reservation:approve".to_string(), "reservation:approve".to_string()));
    reservation_approve_listener.listen(room_repo_listen);
    rocket::build()
        .attach(Shield::default().enable(permission))
        .manage(room_use_case)
        .manage(cors.to_cors())
        .mount(
            "/api/v1",
            routes![
                new_building,
                fetch_all_building,
                fetch_one_building,
                update_building,
                delete_building,
                new_floor,
                fetch_all_floor,
                fetch_one_floor,
                update_floor,
                delete_floor,
                new_room,
                fetch_all_room,
                fetch_one_room,
                update_room,
                delete_room
                
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
