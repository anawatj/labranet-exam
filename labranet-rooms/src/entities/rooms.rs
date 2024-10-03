use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub _id : ObjectId,
    pub floor_id:ObjectId,
    pub room_number:String,
    pub name:String,
    pub price:f64,
    pub is_reservation:bool,
    pub create_by:ObjectId
}