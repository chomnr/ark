use app::arc::ArcDatabase;

use crate::app::arc::ArcServer;

pub mod app;

#[tokio::main]
async fn main() {
    let mut db = ArcDatabase::new();
    db.inject_builders().await;
    
    let arc = ArcServer::default();
    arc.run_http_server().await.unwrap();
    
    //perform health
    // 
    // order:
    // 1. check if databases are good.
    // 2. check if sectors are good?

    // sector system
    // used to ping and communicate with a server via grpc.
    // sector.ping("iam")
    // sector.ping("iam").run("tests").execute():
    //db.sector("cache").run()
    //let arc = ArcServer::default();

    //arc.run_http_server().await.unwrap();
}

//println!("{} running on {}", PREFIX, self.address);

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