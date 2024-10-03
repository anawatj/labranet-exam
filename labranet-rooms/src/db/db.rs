use std::env;
use mongodb::{ options::ClientOptions, Client, Database};
use rocket::fairing::AdHoc;

#[derive(Debug, Clone)]

pub struct  MongoDB {
    pub(crate) database: Database,
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(MongoDB::new(database)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

pub async fn connect() -> mongodb::error::Result<Database> {
    //dotenv().ok();
    println!("{}","Begin Connect" );
    let db_url = env::var("MONGO_URI").expect("Database url not set");
    let client_options = ClientOptions::parse(db_url).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    /*client
        .database("admin")
        .run_command(bson::doc! {"ping": 1}, None)
        .await?;*/

    println!("connected to DB");

    Ok(client.database("rooms"))
}
impl  MongoDB {
    pub fn new(database:Database)->Self{
        MongoDB{database}
    }
}