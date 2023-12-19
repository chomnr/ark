use app::sector::Sector;

pub mod app;

#[tokio::main]
async fn main() {
    let mut sector = Sector::default();
    sector.create_sector("IAM", "/auth", vec![]);
    sector.create_sector("Dashboard", "/dashboard", vec![]);


    //arc.attach_sector(sector);
    //arc.run();

    //let arc = ArcServer::default();

    //arc.run().await
    //let mut arc = ArcServer::default();
    //arc.attach_sector(IamSector::default());

    // start the monstrosity
    //arc.run().await;
    
    //let arc = ArcServer::default();
    // axum setup.
    // inject bb8 builders into ArcDatabase.    
    //let db_inj = db.inject();
    //db.inject().await;
    /*
    db.
    //db.inject_builders().await;
    //db.check_schemas().then(|x| db.load_schema(x));
    
    let arc = ArcServer::default();
    arc.run().await.unwrap();
    */
    
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