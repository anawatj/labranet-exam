use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
   
}