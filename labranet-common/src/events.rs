use nats::{jetstream::Error, Connection, Handler, Subscription};
use rocket::local::blocking::Client;


pub struct Listener {
    pub client:Connection,
    pub subject:String,
    pub queue_group_name:String
}
impl Listener {
    pub fn new(client:Connection,subject:String,queue_group_name:String)->Self{
        Listener{client,subject,queue_group_name}
    }
}

pub struct Publisher {
    pub client:Connection,
    pub subject:String,
}
impl Publisher {
    pub fn new (client:Connection,subject:String)->Self{
        Publisher{client,subject}
    }
}