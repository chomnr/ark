#![forbid(unsafe_code)]
use app::{arc::ArcServer, service::iam::{identity::model::UserIdentity, access::model::UserAccess, account::{model::UserAccount, repository::{UserRepository, UserInsertionField}}}, database::postgres::{PostgresDatabase, PostgresConfig}};

pub mod app;

#[tokio::main]
async fn main() {
    let arc = ArcServer::default();
    let database = PostgresDatabase::new(PostgresConfig::default());
    
    let mut one = UserIdentity::new().build();
    let mut two = UserAccess::new().build();
    let three = UserAccount::new(one, two);

    // user repository
    /*
    UserRepository::insert(three)
        .modify(UserInsertionField::All)
        .execute_on(database)
        .await;
    */

    UserRepository::insert(three)
        .modify(UserInsertionField::All)
        .execute_on(database)
        .await;

    // User Repo\
    /*
    UserRepository::insert(three)
        .modify(UserInsertionField::All)
        .execute_on(database)
        .await;
    */
        //.execute_on(pg);

    //UserRepository::insert(three)
      //  .modify(UserInsertionField::All);

    //let mut three = UserAccount::new(one, two);
    //UserAccount::new(one, two)
        //.create();
    /*
    //insert
    UserAccount::insert()
        .modify(UserField::Username)
        .field(&[&"username"])
        .execute();
    //update
    UserAccount::update(one)
        .modify(&[&UserField::Password, UserField::Username])
        .value(&[&"adsadsasdsa", "asddasadssda"])
        .execute();
    //find
    UserAccount::find()
        .find(UserFind::Username)
        .value("dasdasd")
        .execute();
    */

    //one.email("{username}@{email}.com").build();
    //two.role("admin").build();
    /*
    let testie = UserIdentity::new()
        .email("email")
        .oauth_id("oauth_id")
        .oauth_provider("oauth_provider")
        .username("username")
        .verified(false)
        .build();

    UserIdentityQuery::create_account(testie).on_success(() {
        Ok() => UserIdentityQuery::create_session(testie);;
        Err() => Failed
    });
    UserIdentityQuery::create_session(testie);
    */
    /*
    let test = UserIdentity::new()
        .username("username")
        .email("username@gmail.com")
        .oauth_provider("discord")
        .oauth_id("132132132312312")
        .verified(false)
        .build();
    */
    arc.run().await;
}

/*
    arc.check(Health::Database, () => {
        arc.run():
    })
*/

//const ARC_SERVER_ADDRESS: &str = "0.0.0.0:3000";

// routes
/*
let app: Router = Router::new()
    .route("/", get(|| async { "Hello, World!" }));

let listener = tokio::net::TcpListener::bind(ARC_SERVER_ADDRESS).await.unwrap();
axum::serve(listener, app).await.unwrap();
*/

/*
// sector
let mut sector = Sector::default();
sector.create_sector("IAM", "/auth", vec![
    Arc::new(AuthPartial::new()),
]);
*/

// arc
//arc.attach_sector(sector);
//arc.run();
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
