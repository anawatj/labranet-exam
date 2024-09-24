use serde::Serialize;

#[derive(Serialize,Debug)]
pub enum ResponseBody<T> {
    Data(T)
}
#[derive(Serialize,Debug)]
pub enum ResponseErrorBody<T>{
    Error(T)
}

#[derive(Serialize,Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub body: ResponseBody<T>,
}

#[derive(Serialize,Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResponseError<T>{
    pub error:ResponseErrorBody<T>,
}