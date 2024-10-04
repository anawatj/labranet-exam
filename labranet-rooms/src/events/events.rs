
use core::str;
use std::str::FromStr;

use labranet_common::events::Listener;
use mongodb::bson::oid::ObjectId;
use nats::Handler;
use rocket::futures::FutureExt;
use serde::{Deserialize, Serialize};

use crate::{entities::rooms::Room, repositories::rooms::{RoomRepo, RoomRepoTrait}};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReservationItem {
    pub room:String 
}

pub trait ReservationApproveListen {
    fn listen(&self,repo:Box<dyn RoomRepoTrait>)->Handler;
}
impl ReservationApproveListen for Listener{
  
    fn listen(&self,repo:Box<dyn RoomRepoTrait>)->Handler {
       let handler = self.client.subscribe(&self.subject).unwrap().with_handler(move |m| {
            let data = m.data.clone();
            let s = match String::from_utf8(data) {
                Ok(ss)=>Some(ss) ,
                Err(_)=>None
            };
            if s.is_some(){
                let results = match serde_json::from_str::<Vec<ReservationItem>>(s.unwrap().as_str()) {
                    Ok(j)=>Some(j),
                    Err(_)=>None
                };
                if results.is_some() {
                    let reservations = results.unwrap();
                    for reservation in reservations{
                        let future_room = repo.find_one(ObjectId::from_str(reservation.room.as_str()).unwrap());
                        future_room.map(|option_room|{
                            if option_room.is_some(){
                                let room_db = option_room.unwrap();
                                let room = Room{
                                    _id:room_db._id,
                                    floor_id:room_db.floor_id,
                                    room_number:room_db.room_number,
                                    name:room_db.name,
                                    price:room_db.price,
                                    is_reservation:true,
                                    create_by:room_db.create_by
                                };
                                repo.update(room,room_db._id);
                                ()
                            }else{
                                ()
                            }
                        });
                    }
                }
            }
            m.ack()
       });
       handler
    }
}