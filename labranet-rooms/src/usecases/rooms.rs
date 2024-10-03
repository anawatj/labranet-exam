use labranet_common::{
    jwt::JWT,
    response::{Response, ResponseBody, ResponseError, ResponseErrorBody},
    roles::Role,
};
use mongodb::bson::oid::ObjectId;
use rocket::{
    async_trait,
    http::Status,
    response::status::{Created, Custom},
};
use std::{result, str::FromStr};

use crate::{
    entities::{buildings::Building, floors::Floor, rooms::Room},
    models::{buildings::BuildingModel, floors::FloorModel, rooms::RoomModel},
    repositories::{buildings::BuildingRepoTrait, floors::FloorRepoTrait, rooms::RoomRepoTrait},
};

#[async_trait]
pub trait RoomUseCaseTrait: Send + Sync {
    async fn new_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: BuildingModel,
    ) -> Result<Created<String>, Custom<String>>;
    async fn new_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: FloorModel,
    ) -> Result<Created<String>, Custom<String>>;
    async fn new_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: RoomModel,
    ) -> Result<Created<String>, Custom<String>>;

    async fn fetch_all_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;

    async fn fetch_all_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;
    async fn fetch_all_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>>;

    async fn fetch_one_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn fetch_one_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn fetch_one_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn update_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: BuildingModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn update_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: FloorModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn update_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: RoomModel,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn delete_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;
    async fn delete_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;

    async fn delete_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>;
}
pub struct RoomUseCase {
    building_repo: Box<dyn BuildingRepoTrait>,
    floor_repo: Box<dyn FloorRepoTrait>,
    room_repo: Box<dyn RoomRepoTrait>,
}
impl RoomUseCase {
    pub fn new(
        building_repo: Box<dyn BuildingRepoTrait>,
        floor_repo: Box<dyn FloorRepoTrait>,
        room_repo: Box<dyn RoomRepoTrait>,
    ) -> Self {
        RoomUseCase {
            building_repo,
            floor_repo,
            room_repo,
        }
    }
    pub fn validate_building(&self, model: BuildingModel) -> Vec<String> {
        let errors = [match model.name == "" {
            true => Some("Name must be provided".to_string()),
            false => None,
        }]
        .to_vec();
        errors
            .iter()
            .map(|error| error.to_owned().to_owned())
            .flatten()
            .collect::<Vec<String>>()
    }
    pub fn validate_floor(&self, model: FloorModel) -> Vec<String> {
        let errors = [
            match ObjectId::from_str(&model.building_id).is_err() {
                true => Some("building must be provided".to_string()),
                false => None,
            },
            match model.name == "" {
                true => Some("building name must be provided".to_string()),
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
    pub fn validate_room(&self, model: RoomModel) -> Vec<String> {
        let errors = [
            match ObjectId::from_str(&model.floor_id).is_err() {
                true => Some("floor must be provided".to_string()),
                false => None,
            },
            match model.room_number == "" {
                true => Some("room number must be provided".to_string()),
                false => None,
            },
            match model.price <= 0.0 {
                true => Some("room price must be provided".to_string()),
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
impl RoomUseCaseTrait for RoomUseCase {
    async fn new_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: BuildingModel,
    ) -> Result<Created<String>, Custom<String>> {
        match key {
            Ok(_k) => {
                let errors = self.validate_building(model.clone());
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
                        let building = Building {
                            _id: ObjectId::new(),
                            name: model.name,
                            create_by: ObjectId::from_str(&_k.claims.subject_id.as_str()).unwrap(),
                        };
                        let insert_result = self.building_repo.add(building).await;
                        let result = self
                            .building_repo
                            .find_one(insert_result.inserted_id.as_object_id().unwrap())
                            .await
                            .unwrap();
                        let response = Response {
                            body: ResponseBody::<Building>::Data(result),
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
    async fn new_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: FloorModel,
    ) -> Result<Created<String>, Custom<String>> {
        match key {
            Ok(_k) => {
                let errors = self.validate_floor(model.clone());
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
                        let floor = Floor {
                            _id: ObjectId::new(),
                            building_id: ObjectId::from_str(model.building_id.as_str()).unwrap(),
                            name: model.name,
                            create_by: ObjectId::from_str(_k.claims.subject_id.as_str()).unwrap(),
                        };
                        let insert_result = self.floor_repo.add(floor).await;
                        let result = self
                            .floor_repo
                            .find_one(insert_result.inserted_id.as_object_id().unwrap())
                            .await
                            .unwrap();
                        let response = Response {
                            body: ResponseBody::<Floor>::Data(result),
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
    async fn new_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: RoomModel,
    ) -> Result<Created<String>, Custom<String>> {
        match key {
            Ok(_k) => {
                let errors = self.validate_room(model.clone());
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
                        let room = Room {
                            _id: ObjectId::new(),
                            floor_id: ObjectId::from_str(model.floor_id.as_str()).unwrap(),
                            room_number: model.room_number,
                            name: model.name,
                            price: model.price,
                            is_reservation: false,
                            create_by: ObjectId::from_str(_k.claims.subject_id.as_str()).unwrap(),
                        };
                        let insert_result = self.room_repo.add(room).await;
                        let result = self
                            .room_repo
                            .find_one(insert_result.inserted_id.as_object_id().unwrap())
                            .await
                            .unwrap();
                        let response = Response {
                            body: ResponseBody::<Room>::Data(result),
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

    async fn fetch_all_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
                let results = self.building_repo.find_all().await;
                match results.len() == 0 {
                    true => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error(
                                "Not Found Building".to_string(),
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
                            body: ResponseBody::<Vec<Building>>::Data(results),
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

    async fn fetch_all_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
                let results = self.floor_repo.find_all().await;
                match results.len() == 0 {
                    true => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error(
                                "Not Found Floor".to_string(),
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
                            body: ResponseBody::<Vec<Floor>>::Data(results),
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

    async fn fetch_all_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
                let results = self.room_repo.find_all().await;
                match results.len() == 0 {
                    true => {
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error(
                                "Not Found Floor".to_string(),
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
                            body: ResponseBody::<Vec<Room>>::Data(results),
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

    async fn fetch_one_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
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
                        let result = self
                            .building_repo
                            .find_one(ObjectId::from_str(_id).unwrap())
                            .await;
                        match result {
                            Some(res) => {
                                let response = Response {
                                    body: ResponseBody::<Building>::Data(res),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            }
                            None => {
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Building".to_string(),
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

    async fn fetch_one_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
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
                        let result = self
                            .floor_repo
                            .find_one(ObjectId::from_str(_id).unwrap())
                            .await;
                        match result {
                            Some(res) => {
                                let response = Response {
                                    body: ResponseBody::<Floor>::Data(res),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            }
                            None => {
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Floor".to_string(),
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

    async fn fetch_one_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
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
                        let result = self
                            .room_repo
                            .find_one(ObjectId::from_str(_id).unwrap())
                            .await;
                        match result {
                            Some(res) => {
                                let response = Response {
                                    body: ResponseBody::<Room>::Data(res),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            }
                            None => {
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Room".to_string(),
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

    async fn update_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: BuildingModel,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
                let errors = self.validate_building(model.clone());
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
                            },
                            false => {
                                let result = self.building_repo.find_one(id.clone().unwrap()).await;
                                match result {
                                    Some(building_db) => {
                                        let building = Building {
                                            _id: building_db._id,
                                            name: model.name,
                                            create_by: building_db.create_by,
                                        };
                                        self.building_repo
                                            .update(building, id.clone().unwrap())
                                            .await;
                                        let result = self
                                            .building_repo
                                            .find_one(building_db._id)
                                            .await
                                            .unwrap();
                                        let response = Response {
                                            body: ResponseBody::<Building>::Data(result),
                                        };
                                        Ok(serde_json::to_string(&response).unwrap())
                                    },
                                    None => {
                                        let response = ResponseError {
                                            error: ResponseErrorBody::<String>::Error(
                                                "Not Found Building".to_string(),
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

    async fn update_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: FloorModel,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
                let errors = self.validate_floor(model.clone());
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
                            },
                            false => {
                                let result = self.floor_repo.find_one(id.clone().unwrap()).await;
                                match result {
                                    Some(floor_db) => {
                                        let floor = Floor {
                                            _id: floor_db._id,
                                            building_id:ObjectId::from_str(model.building_id.as_str()).unwrap(),
                                            name: model.name,
                                            create_by: floor_db.create_by,
                                        };
                                        self.floor_repo
                                            .update(floor, id.clone().unwrap())
                                            .await;
                                        let result = self
                                            .floor_repo
                                            .find_one(floor_db._id)
                                            .await
                                            .unwrap();
                                        let response = Response {
                                            body: ResponseBody::<Floor>::Data(result),
                                        };
                                        Ok(serde_json::to_string(&response).unwrap())
                                    },
                                    None => {
                                        let response = ResponseError {
                                            error: ResponseErrorBody::<String>::Error(
                                                "Not Found Floor".to_string(),
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


    async fn update_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        model: RoomModel,
        _id: &str,
    ) -> Result<String, Custom<String>> {
        match key {
            Ok(_k) => {
                let errors = self.validate_room(model.clone());
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
                            },
                            false => {
                                let result = self.room_repo.find_one(id.clone().unwrap()).await;
                                match result {
                                    Some(room_db) => {
                                        let room =Room{
                                            _id:room_db._id,
                                            floor_id:ObjectId::from_str(model.floor_id.as_str()).unwrap(),
                                            room_number:model.room_number,
                                            name:model.name,
                                            price:model.price,
                                            is_reservation:room_db.is_reservation,
                                            create_by:room_db.create_by
                                        };
                                        self.room_repo
                                            .update(room, id.clone().unwrap())
                                            .await;
                                        let result = self
                                            .room_repo
                                            .find_one(room_db._id)
                                            .await
                                            .unwrap();
                                        let response = Response {
                                            body: ResponseBody::<Room>::Data(result),
                                        };
                                        Ok(serde_json::to_string(&response).unwrap())
                                    },
                                    None => {
                                        let response = ResponseError {
                                            error: ResponseErrorBody::<String>::Error(
                                                "Not Found Room".to_string(),
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

    async fn delete_building(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>{
        match key {
            Ok(_k)=>{
                let id = ObjectId::parse_str(_id);
                match id.is_err() {
                    true=>{
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error("Invalid id".to_string()),
                        };
                        Err(Custom(
                            Status {
                                code: Status::BadRequest.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    },
                    false=>{
                        let result = self.building_repo.find_one(id.clone().unwrap()).await;
                        match result {
                            Some(res)=>{
                                self.building_repo.delete(res._id).await;
                                let response = Response {
                                    body: ResponseBody::<String>::Data(
                                        "Delete Success".to_string(),
                                    ),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            },
                            None=>{
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Building".to_string(),
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
            },
            _ =>{ 
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

    async fn delete_floor(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>{
        match key {
            Ok(_k)=>{
                let id = ObjectId::parse_str(_id);
                match id.is_err() {
                    true=>{
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error("Invalid id".to_string()),
                        };
                        Err(Custom(
                            Status {
                                code: Status::BadRequest.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    },
                    false=>{
                        let result = self.floor_repo.find_one(id.clone().unwrap()).await;
                        match result {
                            Some(res)=>{
                                self.floor_repo.delete(res._id).await;
                                let response = Response {
                                    body: ResponseBody::<String>::Data(
                                        "Delete Success".to_string(),
                                    ),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            },
                            None=>{
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Floor".to_string(),
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
            },
            _ =>{ 
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

    async fn delete_room(
        &self,
        key: Result<JWT, ResponseError<String>>,
        _id: &str,
    ) -> Result<String, Custom<String>>{
        match key {
            Ok(_k)=>{
                let id = ObjectId::parse_str(_id);
                match id.is_err() {
                    true=>{
                        let response = ResponseError {
                            error: ResponseErrorBody::<String>::Error("Invalid id".to_string()),
                        };
                        Err(Custom(
                            Status {
                                code: Status::BadRequest.code,
                            },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    },
                    false=>{
                        let result = self.room_repo.find_one(id.clone().unwrap()).await;
                        match result {
                            Some(res)=>{
                                self.room_repo.delete(res._id).await;
                                let response = Response {
                                    body: ResponseBody::<String>::Data(
                                        "Delete Success".to_string(),
                                    ),
                                };
                                Ok(serde_json::to_string(&response).unwrap())
                            },
                            None=>{
                                let response = ResponseError {
                                    error: ResponseErrorBody::<String>::Error(
                                        "Not Found Room".to_string(),
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
            },
            _ =>{ 
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
