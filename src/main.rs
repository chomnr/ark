use crate::app::arc::ArcServer;

pub mod app;

#[tokio::main]
async fn main() {
    //let arc_database = ArcDatabase::default();
    let arc = ArcServer::default();
    //arc.attach_db_instance(database)
     
    //let databse = ArcDatabase::new();
    //database.redis.pool;
    //let arc = ArcServer::default();
    //setup tarpc
    //setup redis
    //setup postgres
    //schema generator
    //master server has /auth/login and /auth/callback.
    //thats it. Checking for pg.schema.sql..
    println!("Hello, woddrld!");
}