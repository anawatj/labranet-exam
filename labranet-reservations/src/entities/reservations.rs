use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReservationItem {
    pub room:String,
    pub price:f64
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reservation {
    pub _id :ObjectId,
    pub reservation_name:String,
    pub description:String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub reservation_date:DateTime,
    pub reservation_status:String,
    pub items:Vec<ReservationItem> ,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub reservation_start_date:DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub reservation_end_date:DateTime,
    pub created_by :ObjectId

}