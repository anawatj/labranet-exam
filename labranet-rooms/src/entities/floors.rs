use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Floor {
    pub _id : ObjectId,
    pub building_id:ObjectId,
    pub name:String,
    pub create_by:ObjectId
}