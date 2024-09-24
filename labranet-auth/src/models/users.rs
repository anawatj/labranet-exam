use rocket::serde::{Deserialize, Serialize};
use labranet_common::roles::Role;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub email: String,
    pub mobile: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}
