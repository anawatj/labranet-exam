use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket::async_trait;
use rocket::futures::StreamExt;
use crate::{db::db::MongoDB, entities::users::User};

#[async_trait]
pub trait  UserRepoTrait : Send+Sync {
    async fn add(&self,user:User)->InsertOneResult ;
    async fn find_all(&self)->Vec<User>;
    async fn find_one(&self,_id:ObjectId)->Option<User>;
    async fn update(&self,user:User,_id:ObjectId)->UpdateResult;
    async fn delete(&self,_id:ObjectId)->DeleteResult;
    async fn find_by_email(&self,email:String)->Option<User>;
}
pub struct UserRepo  {
    mongo:MongoDB
}
impl UserRepo {
    pub fn new(mongo:MongoDB)->Self{
        UserRepo{mongo}
    }
}

#[async_trait]
impl UserRepoTrait for UserRepo{

   
    
    async fn add(&self,user:User)->InsertOneResult {
        let col = self.mongo.database.collection::<User>("users");
        let new_user:User=User{
            _id:user._id,
            email:user.email,
            mobile:user.mobile,
            password:user.password,
            first_name:user.first_name,
            last_name:user.last_name,
            role:user.role
        };
        
        col.insert_one(&new_user).await.unwrap()
      
    }
    async fn find_all(&self)->Vec<User> {
        let col = self.mongo.database.collection::<User>("users");
        let mut  cursor = col.find(doc! {}).await.unwrap();
        let mut results :Vec<User>=Vec::new();
        while let Some(result) = cursor.next().await {
           if result.is_ok(){
            results.push(result.unwrap());
           }
            
        }
        results
    }
    async fn find_one(&self,_id:ObjectId)->Option<User> {
        let col = self.mongo.database.collection::<User>("users");
        let result = col.find_one(doc! {"_id":_id}).await.unwrap();
        result
    }
    async fn update(&self,user:User,_id:ObjectId)->UpdateResult {
        let col = self.mongo.database.collection::<User>("users");
        let result = col.update_one(doc! {"_id":_id}, doc!{"$set":doc!{
            "_id":user._id,
            "email":user.email,
            "mobile":user.mobile,
            "password":user.password,
            "first_name":user.first_name,
            "last_name":user.last_name,
            "role":user.role,
        }}).await.unwrap();
        result
    }
    async fn delete(&self,_id:ObjectId)->DeleteResult {
        let col = self.mongo.database.collection::<User>("users");
        let result = col.delete_one(doc! {"_id":_id}).await.unwrap();
        result
    }
    async fn find_by_email(&self,email:String)->Option<User> {
        let col = self.mongo.database.collection::<User>("users");
        let result = col.find_one(doc! {"email":email}).await.unwrap();
        result
    }
}