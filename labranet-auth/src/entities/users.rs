use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub mobile: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}
