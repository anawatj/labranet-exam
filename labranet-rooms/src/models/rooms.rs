

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomModel {
    pub floor_id:String,
    pub room_number:String,
    pub name:String,
    pub price:f64
}