use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Building {
    pub _id : ObjectId,
    pub name : String,
    pub create_by :ObjectId
}