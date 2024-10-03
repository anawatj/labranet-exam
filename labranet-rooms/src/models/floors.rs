use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FloorModel {
    pub building_id:String,
    pub name:String,
    
}