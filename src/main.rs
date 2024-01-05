use app::{
    ark::ArkServer,
    database::postgres::{PostgresConfig, PostgresDatabase}, service::{iam::role::model::Role, cache::model::Cache},
};

pub mod app;

#[tokio::main]
async fn main() {
    let ark = ArkServer::default().await;
    let database = PostgresDatabase::new(PostgresConfig::default()).await;

    
    let role = Role::builder()
        .id(1)
        .name("sadds")
        .build();
    Cache::<Role>::write(role).unwrap();
    
    let read = Cache::<Role>::read(Role::builder().id(1).build()).unwrap();
    println!("{}", read.name);

    let role2 = Role::builder()
        .id(1)
        .name("testupdate")
        .build();
    Cache::<Role>::update(role2).unwrap();

    let read2 = Cache::<Role>::read(Role::builder().id(1).build()).unwrap();
    println!("{}", read2.name);
    
    /*
    Cache::<Role>::write(role).unwrap();

    let read = Cache::<Role>::read(Role::builder().id(1).build()).unwrap();

    println!("{}", read.name);
    */

    //Cache::<Role>::write(value);

    //Cache::<Role>::write(value);
    //Cache::<>::delete(Role::builder().id(3).name("33").build());
    //Cache::<RoleCache>::read("lookup");
    //let mut repo = RoleRepo::new(database);
    /*
    Role::cache(CacheAction::Create, Role::new(0, "Admin"))
        .await
        .unwrap();

    println!("{}", Role::read(0).await.unwrap().name);

    Role::cache(CacheAction::Create, Role::new(1, "Moderator"))
        .await
        .unwrap();

    println!("{}", Role::read(1).await.unwrap().name);

    Role::cache(CacheAction::Create, Role::new(2, "Flutter"))
        .await
        .unwrap();

    println!("{}", Role::read(2).await.unwrap().name);

    Role::cache(CacheAction::Create, Role::new(3, "Ridic"))
        .await
        .unwrap();

    println!("{}", Role::read(3).await.unwrap().name);
    */

    /*
    if let Err(e) = repo.action(RoleAction::Create).parameter(&[&"Admin"]).execute().await {
        eprintln!("[ARC] {}", e);
        // Handle the error case
    }
    */

    /*
    if let Err(e) = repo.action(PermissionAction::Create).parameter(&[]).execute().await {
        eprintln!("[ARC] Execution failed: {}", e);
        // Handle the error case
    }
    */
    /*
    repo
        .action(PermissionAction::Create).parameter(&[])
        .parameter(&[])
        .execute().await.unwrap();

    repo
        .action(PermissionAction::Create).parameter(&[])
        .parameter(&[])
        .execute().await.unwrap();
    */
    /*
    PermissionRepoBuilder::new(database)
        .action(PermissionAction::Delete)
        .parameter(&[&"Test Permission", &"test.permission"])
        .execute()
        .await
        .unwrap();
    */
    //PermissionRepoBuilder::new()
    ark.run().await;
}

//let pg = PostgresDatabase::new(PostgresConfig::default()).await;

//let test2 = PermissionManager::new(pg.clone());
//let test = RoleManager::new(pg);

//test2.delete_role_permission(9, 1).await.unwrap();
//test.create_role("admin").await.unwrap();

/*

// UserRepository::call_event("identity_create", serde_json)
    //let role = RoleManager::new(pg);
    let role = RoleManager::new(pg);

    //role.create_role("admin").await.unwrap();
    role.update_role("admin", "hello").await.unwrap();


let one = UserIdentity::new()
    .email("hello@gmail.com")
    .oauth_id("32141341")
    .oauth_provider("discord")
    .username("123312")
    .verified(false)
    .clone()
    .build();


let repo = UserRepository::new(database_1);
repo.create_new_identity(&one).await.unwrap();
*/

//repo.create_new_identity(todo!()).await.unwrap();

// let one = UserIdentity::new().clone().build();
// repo.create_new_identity(&one).await.unwrap();

/*
let one = UserIdentity::new()
    .email("test@gmail.com")
    .oauth_id("1233333333")
    .oauth_provider("discord")
    .username("hello")
    .verified(false)
    .clone()
    .build();

repo.create_new_identity(&one).await.unwrap();
*/
//let mut two = UserAccess::new().build();

//
//let three = UserAccount::new(one, two);

//user_repo.create_new_identity(&one);

//user_repo.create_new_identity(&one).await.unwrap();

/*
UserRepository::insert_mode(three)
    .field(UserInsertionField::Permission)
    .value(&[
        "permission.name",
        "permission.name",
        "permission.name",
        "permission.name",
    ])
    .execute_on(database)
    .await.unwrap();
*/

/*
UserRepository::insert_mode(three)
    .modify(UserInsertionField::Permission)
    .value(&[
        "admin.ban.timeout.1123",
        "admin.ban.timeout.1123",
        "admin.ban.timeout.1123",
    ])
    .execute_on(database)
    .await;
*/

// user repository
/*
UserRepository::insert(three)
    .modify(UserInsertionField::All)
    .execute_on(database)
    .await;
*/

/*
UserRepository::insert_mode(three)
    .modify(UserInsertionField::All)
    .execute_on(database)
    .await;
*/

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
