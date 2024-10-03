use labranet_common::{jwt::JWT, response::ResponseError};
use rocket::{delete, get, post, put, response::status::{Created, Custom}, serde::json::Json, State};

use crate::{models::{buildings::BuildingModel, floors::FloorModel, rooms::RoomModel}, usecases::rooms::RoomUseCaseTrait};


#[post("/buildings", format = "application/json", data = "<building>")]
pub async fn new_building(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,building:Json<BuildingModel>)->Result<Created<String>,Custom<String>>{
   let result =  room_use_case.new_building(key,building.into_inner()).await;
   result
}
#[get("/buildings",format="application/json")]
pub async fn fetch_all_building(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>)->Result<String,Custom<String>>{
    let result = room_use_case.fetch_all_building(key).await;
    result 
}
#[get("/buildings/<building_id>",format="application/json")]
pub async fn fetch_one_building(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,building_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.fetch_one_building(key, building_id).await;
    result
}

#[put("/buildings/<building_id>",format="application/json",data="<building>")]
pub async fn update_building(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,building:Json<BuildingModel>,building_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.update_building(key,building.into_inner(),building_id).await;
    result
}
#[delete("/buildings/<building_id>",format="application/json")]
pub async fn delete_building(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,building_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.delete_building(key, building_id).await;
    result
}


#[post("/floors", format = "application/json", data = "<floor>")]
pub async fn new_floor(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,floor:Json<FloorModel>)->Result<Created<String>,Custom<String>>{
   let result =  room_use_case.new_floor(key,floor.into_inner()).await;
   result
}
#[get("/floors",format="application/json")]
pub async fn fetch_all_floor(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>)->Result<String,Custom<String>>{
    let result = room_use_case.fetch_all_floor(key).await;
    result 
}
#[get("/floors/<floor_id>",format="application/json")]
pub async fn fetch_one_floor(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,floor_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.fetch_one_floor(key, floor_id).await;
    result
}

#[put("/floors/<floor_id>",format="application/json",data="<floor>")]
pub async fn update_floor(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,floor:Json<FloorModel>,floor_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.update_floor(key,floor.into_inner(),floor_id).await;
    result
}
#[delete("/floors/<floor_id>",format="application/json")]
pub async fn delete_floor(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,floor_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.delete_floor(key, floor_id).await;
    result
}


#[post("/rooms", format = "application/json", data = "<room>")]
pub async fn new_room(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,room:Json<RoomModel>)->Result<Created<String>,Custom<String>>{
   let result =  room_use_case.new_room(key,room.into_inner()).await;
   result
}
#[get("/rooms",format="application/json")]
pub async fn fetch_all_room(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>)->Result<String,Custom<String>>{
    let result = room_use_case.fetch_all_room(key).await;
    result 
}
#[get("/rooms/<room_id>",format="application/json")]
pub async fn fetch_one_room(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,room_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.fetch_one_room(key, room_id).await;
    result
}

#[put("/rooms/<room_id>",format="application/json",data="<room>")]
pub async fn update_room(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,room:Json<RoomModel>,room_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.update_room(key,room.into_inner(),room_id).await;
    result
}
#[delete("/rooms/<room_id>",format="application/json")]
pub async fn delete_room(room_use_case:&State<Box<dyn RoomUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,room_id:&str)->Result<String,Custom<String>>{
    let result = room_use_case.delete_room(key, room_id).await;
    result
}
