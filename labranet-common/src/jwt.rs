
use rocket::serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, EncodingKey, Algorithm, Header, Validation,DecodingKey}; // 👈 New!
use jsonwebtoken::errors::{Error, ErrorKind};
use chrono::Utc;
use rocket::request::{Outcome, Request, FromRequest}; // 👈 New!
use rocket::http::Status;
use dotenvy::dotenv;
use std::env;

use crate::response::{ResponseError, ResponseErrorBody};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: String,
    pub subject_email:String,
    pub subject_role:String,
    pub exp: usize
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims
}



pub fn create_jwt(id: String,email:String,role:String) -> Result<String, Error> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set."); // 👈 New!

    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).expect("Invalid timestamp").timestamp();
    
    let claims = Claims {
        subject_id: id,
        subject_email:email,
        subject_role:role,
        exp: expiration as usize
    }; 

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    // 👇 New!
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}






#[rocket::async_trait]
impl <'r> FromRequest<'r> for JWT{
    type Error = ResponseError<String>;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ResponseError<String>>{
        fn is_valid(key: &str) -> Result<Claims,Error> {
            Ok(decode_jwt(String::from(key))?)
        }
        match req.headers().get_one("authorization") {
            None => {
                let response = ResponseError { error: ResponseErrorBody::<String>::Error(String::from("Error validating JWT token - No token provided"))};
                
                Outcome::Error((Status::Unauthorized, response)) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = ResponseError { error: ResponseErrorBody::<String>::Error(format!("Error validating JWT token - Expired Token"))};
                        Outcome::Error((Status::Unauthorized, response))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = ResponseError { error: ResponseErrorBody::<String>::Error(format!("Error validating JWT token - Invalid Token"))};
                        Outcome::Error((Status::Unauthorized, response)) 
                    },
                    _ => {
                        let response = ResponseError { error: ResponseErrorBody::<String>::Error(format!("Error validating JWT token - {}", err))};
                        Outcome::Error((Status::Unauthorized, response)) 
                    }
                }
            },
        }
    }
}