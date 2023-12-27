
/*
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