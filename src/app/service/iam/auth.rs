use serde::{Serialize, Deserialize};

use super::{user::User, permission::Permission, role::Role};

/// Represents the authentication and authorization details of a user.
///
/// This struct combines user identity information with their associated permissions and roles within the system.
///
/// Fields:
/// - `user`: A `User` struct representing the user's basic identity and account details.
/// - `perm`: A vector of `Permission` structs detailing the specific permissions granted to the user.
/// - `role`: A vector of `Role` structs representing the roles assigned to the user.
///
/// The `UserAuth` struct is useful for encapsulating all the authentication and authorization-related information of a user, enabling easy access and management of these attributes.
#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    info: User,
    perm: Vec<Permission>,
    role: Vec<Role>
}

impl UserAuth {
    
}