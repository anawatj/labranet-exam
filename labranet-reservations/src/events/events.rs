use labranet_common::events::Publisher;

use crate::entities::reservations::ReservationItem;


pub trait ReservationApprovePublish {
    
    fn publish(&self,data:Vec<ReservationItem>);
}

impl  ReservationApprovePublish for Publisher {
    fn publish(&self,data:Vec<ReservationItem>) {
        self.client.publish(&self.subject, serde_json::to_string(&data).unwrap().as_str().as_bytes())
    }
    
}