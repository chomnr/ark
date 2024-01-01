pub mod iam;
pub mod integrity;

/*
#[proc_macro_attribute]
pub fn permission(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);

    // here you'd parse 'args' and do something meaningful with it

    let result = quote! {
        #function
    };

    result.into()
}
*/

/*
#[proc_macro_attribute]
pub fn permission(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);

    // here you'd parse 'args' and do something meaningful with it

    let result = quote! {
        #function
    };

    result.into()
}
*/

/*
pub struct User {
    info: UserInfo,
    perm: Vec<Permission>,
    role: Vec<Role>,
}

pub struct UserInfo {
    id: i32,
    username: String,
    email: String,
    verified: bool,
    created_at: String,
    updated_at: String,
}

impl User {
    pub fn new(
        username: &str,
        email: &str,
        verified: bool,
        perm: Vec<Permission>,
        role: Vec<Role>,
    ) -> Self {
        Self {
            info: UserInfo {
                id: 0,
                username: String::from(username),
                email: String::from(email),
                verified,
                created_at: String::default(),
                updated_at: String::default(),
            },
            perm,
            role,
        }
    }
}

pub struct UserManager {
    pg: PostgresDatabase,
}

impl UserManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub async fn create_user(&self, id: i32) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = todo!();
        todo!()
    }

    pub async fn set_username(&self, id: i32, username: &str) {}

    pub async fn set_email(&self, id: i32, email: &str) {}

    pub async fn set_verified(&self, verified: bool) {}
}
*/

//let stmt = pool.prepare("INSERT INTO users (username, email, verified) VALUES($1, $2, $3)").await?;
// create user
/*
let stmt = pool
    .prepare(
        "INSERT INTO identity (username, email, verified, created_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (oauth_id)
        DO UPDATE SET updated_at = CURRENT_TIMESTAMP
        RETURNING *;",
    )
    .await?;
*/
//let result = pool.execute(&stmt, &[&role_id, &permission_id]).await?;
//Ok(result)

/*

fn test(){
    //UserManager::new(pg)
      //  .create_user();
}



pub struct User {
    id: i32,
    username: String,
    email: String,
    verified: bool,
    created_at: String,
    updated_at: String,
    perms: Vec<Permission>,
    role: Role,
}

impl User {
    pub fn new(username: &str, email: &str, perms: Vec<Permission>, role: Role) -> Self {
        Self {
            id: i32::default(),
            username: String::from(username),
            email: String::from(email),
            verified: false,
            created_at: String::default(),
            updated_at: String::default(),
            perms,
            role,
        }
    }
}

pub struct UserManager {
    pg: PostgresDatabase
}

impl UserManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self {
            pg
        }
    }
}
*/
