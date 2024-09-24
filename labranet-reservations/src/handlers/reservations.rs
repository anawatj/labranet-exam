use std::result;

use labranet_common::{jwt::JWT, response::ResponseError};
use rocket::{delete, get, post, put, response::status::{Created, Custom}, serde::json::Json, State};

use crate::{models::reservations::ReservationModel, usecases::reservations::ReservationUseCaseTrait};

#[post("/reservations", format = "application/json", data = "<reservation>")]
pub async fn new_reservation(reservation_use_case:&State<Box<dyn ReservationUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,reservation:Json<ReservationModel>)->Result<Created<String>,Custom<String>>{
   let result =  reservation_use_case.new_reservation(key, reservation.into_inner()).await;
   result
}
#[get("/reservations",format="application/json")]
pub async fn fetch_all_reservation(reservation_use_case:&State<Box<dyn ReservationUseCaseTrait>>,key:Result<JWT,ResponseError<String>>)->Result<String,Custom<String>>{
    let result = reservation_use_case.fetch_all_reservation(key).await;
    result
}
#[get("/reservations/<reservation_id>",format="application/json")]
pub async fn fetch_one_reservation(reservation_use_case:&State<Box<dyn ReservationUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,reservation_id:&str)->Result<String,Custom<String>>{
    let result = reservation_use_case.fetch_one_reservation(key, reservation_id).await;
    result
}
#[put("/reservations/<reservation_id>",format="application/json",data="<reservation>")]
pub async fn update_reservation(reservation_use_case:&State<Box<dyn ReservationUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,reservation:Json<ReservationModel>,reservation_id:&str)->Result<String,Custom<String>>{
    let result = reservation_use_case.update_reservation(key,reservation.into_inner(),reservation_id).await;
    result
}

#[delete("/reservations/<reservation_id>",format="application/json")]
pub async fn delete_reservation(reservation_use_case:&State<Box<dyn ReservationUseCaseTrait>>,key:Result<JWT,ResponseError<String>>,reservation_id:&str)->Result<String,Custom<String>>{
    let result = reservation_use_case.delete_reservation(key,reservation_id).await;
    result
}