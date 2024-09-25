use std::str::FromStr;

use chrono::DateTime;
use labranet_common::{
    jwt::JWT,
    response::{Response, ResponseBody, ResponseError, ResponseErrorBody},
};
use mongodb::bson::oid::ObjectId;
use rocket::{
    async_trait,
    http::Status,
    response::status::{Created, Custom},
};

use crate::{
    entities::reservations::{Reservation, ReservationItem},
    models::reservations::{ReservationModel, ReservationStatus},
    repositories::reservations::ReservationRepoTrait,
};

#[async_trait]
pub trait ReservationUseCaseTrait: Send + Sync {
    async fn new_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: ReservationModel,
    ) -> Result<Created<String>, Custom<String>>;
    async fn fetch_all_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;
    async fn fetch_one_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn update_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: ReservationModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn delete_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;
}

pub struct ReservationUseCase {
    pub repo: Box<dyn ReservationRepoTrait>,
}
impl ReservationUseCase {
    pub fn new(repo: Box<dyn ReservationRepoTrait>) -> Self {
        ReservationUseCase { repo }
    }
    fn validate_reservation(&self, model: ReservationModel) -> Vec<String> {
        let errors = [
            match model.reservation_name == "" {
                true => Some("Reservation name is required".to_string()),
                false => None,
            },
            match model.description == "" {
                true => Some("Description is required".to_string()),
                false => None,
            },
            match model.items.is_empty() {
                true => Some("Reservation Item is required".to_string()),
                false => None,
            },
        ]
        .to_vec();
        errors
            .iter()
            .map(|error| error.to_owned().to_owned())
            .flatten()
            .collect::<Vec<String>>()
    }
}
#[async_trait]
impl ReservationUseCaseTrait for ReservationUseCase {
    async fn new_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: ReservationModel,
    ) -> Result<Created<String>, Custom<String>> {
        match key {
            Ok(k) => {
                let errors = self.validate_reservation(model.clone());
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
                    }
                    false => {
                        let reservation = Reservation {
                            _id: ObjectId::new(),
                            reservation_name: model.reservation_name,
                            description: model.description,
                            reservation_date: model.reservation_date,
                            reservation_status: ReservationStatus::Save.to_string(),
                            reservation_end_date: model.reservation_start_date,
                            reservation_start_date: model.reservation_end_date,
                            items: model
                                .items
                                .iter()
                                .map(|item| ReservationItem {
                                    price: item.clone().price,
                                    room: item.clone().room,
                                })
                                .collect::<Vec<ReservationItem>>(),
                            created_by: ObjectId::from_str(k.claims.subject_id.as_str()).unwrap(),
                        };
                        let insert_result = self.repo.add(reservation).await;
                        let result = self
                            .repo
                            .find_one(insert_result.inserted_id.as_object_id().unwrap())
                            .await
                            .unwrap();
                        let response = Response {
                            body: ResponseBody::<Reservation>::Data(result),
                        };
                        Ok(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
                    }
                }
            }
            _ => {
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error("Unauthorize".to_string()),
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

    async fn fetch_all_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(k) => {
                let results = self.repo.find_all().await;
                match results.len() == 0 {
                    true => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error(
                                "Not Found Reservations".to_string(),
                            ),
                        };
                        Err(Custom(
                            Status {
                                code: Status::NotFound.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    false => {
                        let response = Response {
                            body: ResponseBody::<Vec<Reservation>>::Data(results),
                        };
                        Ok(serde_json::to_string(&response).unwrap())
                    }
                }
            }
            _ => {
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error("Unauthorize".to_string()),
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
    async fn fetch_one_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(k) => {
                let id = ObjectId::parse_str(_id);
                match id.clone().is_err() {
                    true => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error("Invalid id".to_string()),
                        };
                        Err(Custom(
                            Status {
                                code: Status::BadRequest.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    false => {
                        println!("{}", id.clone().unwrap());
                        let result = self.repo.find_one(id.clone().unwrap()).await;
                        match result {
                            Some(res) => {
                                let response = Response {
                                    body: ResponseBody::<Reservation>::Data(res),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            }
                            None => {
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Reservations".to_string(),
                                    ),
                                };
                                Err(Custom(
                                    Status {
                                        code: Status::NotFound.code,
                                    },
                                    serde_json::to_string(&response).unwrap(),
                                ))
                            }
                        }
                    }
                }
            }
            _ => {
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error("Unauthorize".to_string()),
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
    async fn update_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: ReservationModel,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(k) => {
                let errors = self.validate_reservation(model.clone());
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
                    }
                    false => {
                        let id = ObjectId::parse_str(_id);
                        match id.is_err() {
                            true => {
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Invalid id".to_string(),
                                    ),
                                };
                                Err(Custom(
                                    Status {
                                        code: Status::BadRequest.code,
                                    },
                                    serde_json::to_string(&response).unwrap(),
                                ))
                            }
                            false => {
                                println!("{}", id.clone().unwrap());
                                let result = self.repo.find_one(id.clone().unwrap()).await;
                                match result {
                                    Some(reservation_db) => {
                                        let reservation = Reservation {
                                            _id: reservation_db._id,
                                            reservation_name: model.reservation_name,
                                            description: model.description,
                                            reservation_date: model.reservation_date,
                                            reservation_status: ReservationStatus::Save.to_string(),
                                            reservation_start_date: model.reservation_start_date,
                                            reservation_end_date: model.reservation_end_date,
                                            items: model
                                                .items
                                                .iter()
                                                .map(|item| ReservationItem {
                                                    price: item.clone().price,
                                                    room: item.clone().room,
                                                })
                                                .collect::<Vec<ReservationItem>>(),
                                            created_by: reservation_db.created_by,
                                        };
                                        self.repo.update(reservation, id.clone().unwrap()).await;
                                        let result =
                                            self.repo.find_one(reservation_db._id).await.unwrap();
                                        let response = Response {
                                            body: ResponseBody::<Reservation>::Data(result),
                                        };
                                        Ok(serde_json::to_string(&response).unwrap())
                                    }
                                    None => {
                                        let response = ResponseError {
                                            error: ResponseErrorBody::<String>::Error(
                                                "Not Found Reservations".to_string(),
                                            ),
                                        };
                                        Err(Custom(
                                            Status {
                                                code: Status::NotFound.code,
                                            },
                                            serde_json::to_string(&response).unwrap(),
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error("Unauthorize".to_string()),
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
    async fn delete_reservation(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(k) => {
                let id = ObjectId::parse_str(_id);
                match id.is_err() {
                    true => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error("Invalid id".to_string()),
                        };
                        Err(Custom(
                            Status {
                                code: Status::BadRequest.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    false => {
                        println!("{}", id.clone().unwrap());
                        let result = self.repo.find_one(id.clone().unwrap()).await;
                        match result {
                            Some(res) => {
                                self.repo.delete(res._id).await;
                                let response = Response {
                                    body: ResponseBody::<String>::Data(
                                        "Delete Success".to_string(),
                                    ),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            }
                            None => {
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Reservations".to_string(),
                                    ),
                                };
                                Err(Custom(
                                    Status {
                                        code: Status::NotFound.code,
                                    },
                                    serde_json::to_string(&response).unwrap(),
                                ))
                            }
                        }
                    }
                }
            }
            _ => {
                let response = ResponseError {
                    error: ResponseErrorBody::<String>::Error("Unauthorize".to_string()),
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
}
