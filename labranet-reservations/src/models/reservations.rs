use core::fmt;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationStatus{
    Save=1,
    Complete=2,
    Cancel=3
}
impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)

    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReservationItemModel {
    pub room:String,
    pub price:f64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReservationModel {
    pub reservation_name:String,
    pub description:String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub reservation_date:DateTime,
    pub items:Vec<ReservationItemModel> ,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub reservation_start_date:DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub reservation_end_date:DateTime,

}