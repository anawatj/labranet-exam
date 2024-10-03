use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket::async_trait;
use rocket::futures::StreamExt;

use crate::db::db::MongoDB;
use crate::entities::floors::Floor;

#[async_trait]
pub trait FloorRepoTrait: Send + Sync {
    async fn add(&self, floor: Floor) -> InsertOneResult;
    async fn find_all(&self) -> Vec<Floor>;
    async fn find_one(&self, _id: ObjectId) -> Option<Floor>;
    async fn update(&self, floor: Floor, _id: ObjectId) -> UpdateResult;
    async fn delete(&self, _id: ObjectId) -> DeleteResult;
}

pub struct FloorRepo {
    mongo: MongoDB,
}
impl FloorRepo {
    pub fn new(mongo: MongoDB) -> Self {
        FloorRepo { mongo }
    }
}
#[async_trait]
impl FloorRepoTrait for FloorRepo {
    async fn add(&self, floor: Floor) -> InsertOneResult {
        let col = self.mongo.database.collection::<Floor>("floors");
        let new_floor = Floor {
            _id: floor._id,
            building_id: floor.building_id,
            name: floor.name,
            create_by: floor.create_by,
        };
        col.insert_one(&new_floor).await.unwrap()
    }
    async fn find_all(&self) -> Vec<Floor> {
        let col = self.mongo.database.collection::<Floor>("floors");
        let mut cursor = col.find(doc! {}).await.unwrap();
        let mut results: Vec<Floor> = Vec::new();
        while let Some(result) = cursor.next().await {
            if result.is_ok() {
                results.push(result.unwrap());
            }
        }
        results
    }
    async fn find_one(&self, _id: ObjectId) -> Option<Floor> {
        let col = self.mongo.database.collection::<Floor>("floors");
        let result = col.find_one(doc! {"_id":_id}).await.unwrap();
        result
    }
    async fn update(&self, floor: Floor, _id: ObjectId) -> UpdateResult {
        let col = self.mongo.database.collection::<Floor>("floors");
        col.update_one(
            doc! {"_id":_id},
            doc! {
                "$set":doc!{
                    "building_id":floor.building_id,
                    "name":floor.name
                }
            },
        )
        .await
        .unwrap()
    }
    async fn delete(&self, _id: ObjectId) -> DeleteResult {
        let col = self.mongo.database.collection::<Floor>("floors");
        col.delete_one(doc! {"_id":_id}).await.unwrap()
    }
}
