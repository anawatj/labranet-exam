use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket::async_trait;
use rocket::futures::StreamExt;
use crate::db::db::MongoDB;
use crate::entities::rooms::Room;

#[async_trait]
pub trait  RoomRepoTrait : Send+Sync {
    async fn add(&self,room:Room)->InsertOneResult ;
    async fn find_all(&self)->Vec<Room>;
    async fn find_one(&self,_id:ObjectId)->Option<Room>;
    async fn update(&self,room:Room,_id:ObjectId)->UpdateResult;
    async fn delete(&self,_id:ObjectId)->DeleteResult;
}

pub struct RoomRepo {
    mongo:MongoDB
}
impl  RoomRepo {
    pub fn new(mongo:MongoDB)->Self{
        RoomRepo{mongo}
    }

}
#[async_trait]
impl RoomRepoTrait for RoomRepo {
    async fn add(&self,room:Room)->InsertOneResult {
        let col = self.mongo.database.collection::<Room>("rooms");
        let new_room = Room{
            _id:room._id,
            floor_id:room.floor_id,
            room_number:room.room_number,
            name:room.name,
            price:room.price,
            is_reservation:room.is_reservation,
            create_by:room.create_by
        };
        col.insert_one(&new_room).await.unwrap()
    }
    async fn find_all(&self)->Vec<Room>{
        let col = self.mongo.database.collection::<Room>("rooms");
        let mut cursor = col.find(doc! {}).await.unwrap();
        let mut results:Vec<Room> = Vec::new();
        while let Some(result) = cursor.next().await {
            if result.is_ok() {
                results.push(result.unwrap());
            }
        }
        results
    }
    async fn find_one(&self,_id:ObjectId)->Option<Room>{
        let col = self.mongo.database.collection::<Room>("rooms");
        let result = col.find_one(doc! {"_id":_id}).await.unwrap();
        result
    }
    async fn update(&self,room:Room,_id:ObjectId)->UpdateResult{
        let col = self.mongo.database.collection::<Room>("rooms");
        col.update_one(doc! {"_id":_id},doc! {"$set":doc!{
            "floor_id":room.floor_id,
            "room_number":room.room_number,
            "name":room.name,
            "price":room.price,
            "is_reservation":room.is_reservation
        }}).await.unwrap()
    }
    async fn delete(&self,_id:ObjectId)->DeleteResult{
        let col = self.mongo.database.collection::<Room>("rooms");
        col.delete_one(doc! {"_id":_id}).await.unwrap()
    }
}