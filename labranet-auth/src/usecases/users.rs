use jsonwebtoken::jwk::Jwk;
use labranet_common::jwt::{create_jwt, decode_jwt, JWT};
use labranet_common::response::{Response, ResponseBody, ResponseError, ResponseErrorBody};

use mongodb::bson::oid::ObjectId;
use rocket::async_trait;

use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use labranet_common::roles::Role;
use crate::entities::users::User;
use crate::models::login::LoginModel;
use crate::models::users::UserModel;

use crate::repositories::users::UserRepoTrait;
use crate::utils::password;

#[async_trait]
pub trait UserUseCaseTrait : Send+Sync {
    async fn sign_up(&self, model: UserModel) -> Result<Created<String>, Custom<String>>;
    async fn login(&self,model:LoginModel)->Result<String,Custom<String>>;
    async fn get_current_user(&self,key:Result<JWT,ResponseError<String>>)->Result<String,Custom<String>>;
}
pub struct UserUseCase {
    repo: Box<dyn UserRepoTrait>,
}
impl UserUseCase{
    pub fn new(repo: Box<dyn UserRepoTrait>) -> Self {
        UserUseCase{repo}
    }
    fn map_field(&self, field: String, message: String) -> Option<String> {
        match field == "" {
            true => Some(message.to_string()),
            false => None,
        }
    }
    fn validate_sign_up(&self, model: UserModel) -> Vec<String> {
        let errors = [
            self.map_field(model.email, "Email is required".to_string()),
            self.map_field(model.mobile, "Mobile is required".to_string()),
            self.map_field(model.password, "Password is required".to_string()),
            self.map_field(model.first_name, "First Name is required".to_string()),
            self.map_field(model.last_name, "Last Name is required".to_string()),
            match model.role {
                Role::Admin | Role::Worker=>None,
                _ => Some("Role is required".to_string()),
            },
        ]
        .to_vec();
        errors
            .iter()
            .map(|x| x.to_owned().to_owned())
            .flatten()
            .collect::<Vec<String>>()
    }

    fn validate_log_in(&self,model:LoginModel)->Vec<String> {
        let errors = [
            self.map_field(model.email, "Email is required".to_string()),
            self.map_field(model.password, "Password is required".to_string()),
        ]
        .to_vec();
        errors
            .iter()
            .map(|x| x.to_owned().to_owned())
            .flatten()
            .collect::<Vec<String>>()
    }
}

#[async_trait]
impl UserUseCaseTrait for UserUseCase {
    
   async fn get_current_user(&self ,key:Result<JWT,ResponseError<String>>)->Result<String,Custom<String>>{
     match key {
         Ok(k)=>{
            let result = self.repo.find_by_email(k.claims.subject_email).await;
            match result {
                Some(user)=>{
                    let response = Response {
                        body:ResponseBody::<User>::Data(user)
                    };
                    Ok( serde_json::to_string(&response).unwrap())
                },
                _ => {
                    let response = ResponseError{
                        error:ResponseErrorBody::<String>::Error("Not found User".to_string())
                    };
                    Err(Custom(
                        Status {
                            code: Status::BadRequest.code,
                        },
                        serde_json::to_string(&response).unwrap(),
                    ))
                }
            }
         },
         _ => {
            let response = ResponseError{
                error:ResponseErrorBody::<String>::Error("Unauthorize".to_string())
            };
            Err(Custom(
                Status {
                    code: Status::Unauthorized.code,
                },
                serde_json::to_string(&response).unwrap(),
            ))
         }
     }
   }
    
    async fn sign_up(&self, model: UserModel) -> Result<Created<String>, Custom<String>> {
        let errors = self.validate_sign_up(model.clone());
        match errors.len() > 0 {
            true => {
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error(errors.join(",")),
                };
                Err(Custom(
                    Status {
                        code: Status::BadRequest.code,
                    },
                    serde_json::to_string(&response).unwrap(),
                ))
            },
            false => {
                let exist_user = self.repo.find_by_email(model.email.to_string()).await;
                match exist_user {
                    Some(_) => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error(
                                "User is already exists".to_string(),
                            ),
                        };
                        Err(Custom(
                            Status {
                                code: Status::BadRequest.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    None => {
                        
                        let user = User {
                            _id:ObjectId::new(),
                            email: model.email,
                            mobile: model.mobile,
                            password: password::hash(model.password),
                            first_name: model.first_name,
                            last_name: model.last_name,
                            role: model.role.to_string(),
                        };
                        let insert_result = self.repo.add(user).await;
                        let result = self
                            .repo
                            .find_one(insert_result.inserted_id.as_object_id().unwrap())
                            .await
                            .unwrap();
                        let response = Response {
                            body: ResponseBody::<User>::Data(result),
                        };
                        Ok(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
                    }
                }
            }
        }
    }
    
    async fn login(&self,model:LoginModel)->Result<String,Custom<String>> {
        let errors =self.validate_log_in(model.clone());
        match errors.len()>0 {
            true=>{
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error(errors.join(",")),
                };
                Err(Custom(
                    Status {
                        code: Status::BadRequest.code,
                    },
                    serde_json::to_string(&response).unwrap(),
                ))
            },
            false=>{
                let exist_user = self.repo.find_by_email(model.email.to_string()).await;
                match exist_user {
                    Some(user)=>{
                        match password::verify(model.password.to_string(),user.password) {
                            true=>{
                                match create_jwt(user._id.to_string(), user.email,user.role){
                                    Ok(jwt)=>{
                                        let response = Response {
                                            body: ResponseBody::<String>::Data(jwt.to_string()),
                                        };
                                        Ok(serde_json::to_string(&response).unwrap())
                                    },
                                    _ =>{
                                        let response = ResponseError {
                                            error: ResponseErrorBody::<String>::Error("Login Fail".to_string()),
                                        };
                                        Err(Custom(
                                            Status { code: 401 },
                                            serde_json::to_string(&response).unwrap(),
                                        ))
                                      }
                                }

                            },
                            false=>{
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error("Login Fail".to_string()),
                                };
                                Err(Custom(
                                    Status { code: 401 },
                                    serde_json::to_string(&response).unwrap(),
                                ))
                            }
                        }
                    },
                    _ => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error("Login Fail".to_string()),
                        };
                        Err(Custom(
                            Status { code: 401 },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    
                }
            }
        }
    }
}
