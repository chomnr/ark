use super::role::Role;

struct User {
    // Fields from the 'users' table
    id: i64,
    username: String,
    email: String,
    verified: bool,
    created_at: i64,
    updated_at: i64,

    // Fields from the 'user_oauth' table
    oauth_id: String,
    oauth_provider: String,

    // Roles and Permissions
    roles: Vec<Role>
}

//UserRepo::create_user(user);