use mongodb::bson::{Bson, Document,DateTime};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket::async_trait;
use rocket::futures::StreamExt;
use crate::{db::db::MongoDB, entities::reservations::Reservation};

#[async_trait]
pub trait  ReservationRepoTrait : Send+Sync {
    async fn add(&self,reservation:Reservation)->InsertOneResult ;
    async fn find_all(&self)->Vec<Reservation>;
    async fn find_one(&self,_id:ObjectId)->Option<Reservation>;
    async fn update(&self,reservation:Reservation,_id:ObjectId)->UpdateResult;
    async fn delete(&self,_id:ObjectId)->DeleteResult;
}
pub struct ReservationRepo  {
    mongo:MongoDB
}
impl ReservationRepo {
    pub fn new(mongo:MongoDB)->Self{
        ReservationRepo{mongo}
    }
}

#[async_trait]
impl ReservationRepoTrait for ReservationRepo{

    async fn add(&self,reservation:Reservation)->InsertOneResult{
        let col = self.mongo.database.collection::<Reservation>("reservations");
        let new_reservation=Reservation{
            _id:reservation._id,
            reservation_name:reservation.reservation_name,
            description:reservation.description,
            reservation_date:reservation.reservation_date,
            reservation_start_date:reservation.reservation_start_date,
            reservation_end_date:reservation.reservation_end_date,
            reservation_status:reservation.reservation_status,
            items:reservation.items,
            created_by:reservation.created_by
        };
        
        col.insert_one(&new_reservation).await.unwrap()
    }
    async fn find_all(&self)->Vec<Reservation>{
        let col = self.mongo.database.collection::<Reservation>("reservations");
        let mut  cursor = col.find(doc! {}).await.unwrap();
        let mut results :Vec<Reservation>=Vec::new();
        while let Some(result) = cursor.next().await {
           if result.is_ok(){
            results.push(result.unwrap());
           }
            
        }
        results
    }
    async fn find_one(&self,_id:ObjectId)->Option<Reservation>{
        let col = self.mongo.database.collection::<Reservation>("reservations");
        let result = col.find_one(doc! {"_id":_id}).await.unwrap();
        result
    }
    async fn update(&self,reservation:Reservation,_id:ObjectId)->UpdateResult{
        let reservation_date = DateTime::from_millis(reservation.reservation_date.timestamp_millis());
        let reservation_start_date = DateTime::from_millis(reservation.reservation_start_date.timestamp_millis());
        let reservation_end_date = DateTime::from_millis(reservation.reservation_end_date.timestamp_millis());
        let col = self.mongo.database.collection::<Reservation>("reservations");
        let items = Bson::from(reservation.items.iter().map(|item| Document::from(doc! {"price":item.clone().price,"room":item.clone().room})).collect::<Vec<Document>>());
        let result = col.update_one(doc! {"_id":_id}, doc!{"$set":doc!{
            "reservation_name":reservation.reservation_name,
            "description":reservation.description,
            "reservation_date":reservation_date,
            "reservation_status":reservation.reservation_status,
            "reservation_start_date":reservation_start_date,
            "reservation_end_date":reservation_end_date,
            "items":items
        }}).await.unwrap();
        result
    }
    async fn delete(&self,_id:ObjectId)->DeleteResult{
        let col = self.mongo.database.collection::<Reservation>("reservations");
        let result = col.delete_one(doc! {"_id":_id}).await.unwrap();
        result
    }
    
}