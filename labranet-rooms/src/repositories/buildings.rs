use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket::async_trait;
use rocket::futures::StreamExt;
use crate::{db::db::MongoDB, entities::buildings::Building};

#[async_trait]
pub trait  BuildingRepoTrait : Send+Sync {
    async fn add(&self,building:Building)->InsertOneResult ;
    async fn find_all(&self)->Vec<Building>;
    async fn find_one(&self,_id:ObjectId)->Option<Building>;
    async fn update(&self,building:Building,_id:ObjectId)->UpdateResult;
    async fn delete(&self,_id:ObjectId)->DeleteResult;
}

pub struct BuildingRepo {
    mongo:MongoDB
}
impl  BuildingRepo {
    pub fn new(mongo:MongoDB)->Self{
        return BuildingRepo{mongo}
    }
}
#[async_trait]
impl BuildingRepoTrait for BuildingRepo {
    async fn add(&self,building:Building)->InsertOneResult{
        let col = self.mongo.database.collection::<Building>("buildings");
        let new_building = Building{
            _id:building._id,
            name:building.name,
            create_by:building.create_by
        };
        col.insert_one(&new_building).await.unwrap()
   }
   async fn find_all(&self)->Vec<Building> {
    let col = self.mongo.database.collection::<Building>("buildings");
    let mut  cursor = col.find(doc! {}).await.unwrap();
    let mut results:Vec<Building> = Vec::new();
    while let Some(result) = cursor.next().await {
        if result.is_ok(){
         results.push(result.unwrap());
        }
         
    }
    results 
   }
   async fn find_one(&self,_id:ObjectId)->Option<Building>{
        let col = self.mongo.database.collection::<Building>("buildings");
        let result = col.find_one(doc! {"_id":_id}).await.unwrap();
        result
   }
   async fn update(&self,building:Building,_id:ObjectId)->UpdateResult{
    let col = self.mongo.database.collection::<Building>("buildings");
    let result=col.update_one(doc! {"_id":_id},doc! {
        "$set":doc!{
            "name":building.name
        }
    }).await.unwrap();
    result
   }
   async fn delete(&self,_id:ObjectId)->DeleteResult{
    let col = self.mongo.database.collection::<Building>("buildings");
    col.delete_one(doc! {"_id":_id}).await.unwrap()
   }
}