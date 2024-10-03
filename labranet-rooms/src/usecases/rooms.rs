use labranet_common::{
    jwt::JWT,
    response::{Response, ResponseBody, ResponseError, ResponseErrorBody},
    roles::Role,
};
use mongodb::bson::oid::ObjectId;
use rocket::{
    async_trait,
    http::Status,
    response::status::{Created, Custom},
};
use std::{result, str::FromStr};

use crate::models::{buildings::BuildingModel, floors::FloorModel, rooms::RoomModel};

#[async_trait]
pub trait RoomUseCaseTrait: Send + Sync {
    async fn new_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: BuildingModel,
    ) -> Result<Created<String>, Custom<String>>;
    async fn new_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: FloorModel,
    ) -> Result<Created<String>, Custom<String>>;
    async fn new_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: RoomModel,
    ) -> Result<Created<String>, Custom<String>>;

    async fn fetch_all_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;

    async fn fetch_all_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;
    async fn fetch_all_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;

    async fn fetch_one_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id:&str
    ) -> Result<String, Custom<String>>;
    async fn fetch_one_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id:&str
    ) -> Result<String, Custom<String>>;

    async fn fetch_one_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id:&str
    ) -> Result<String, Custom<String>>;

    async fn update_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: BuildingModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn update_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: FloorModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn update_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: RoomModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn delete_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn delete_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn delete_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;

}
