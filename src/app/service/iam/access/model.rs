/// Represents the access control information for a user.
///
/// Fields:
/// - `role`: Numeric representation of the user's role.
/// - `permission`: A list of strings representing the specific permissions granted to the user.
pub(crate) struct UserAccess {
    role: usize,
    permission: Vec<String>,
}

impl Default for UserAccess {
    fn default() -> Self {
        Self {
            role: Default::default(),
            permission: Default::default(),
        }
    }
}

impl UserAccess {
    pub fn new() -> UserAccessBuilder {
        let def = UserAccess::default();
        UserAccessBuilder {
            role: def.role,
            permission: def.permission,
        }
    }
}

/// Builder for constructing a `UserAccess` instance.
///
/// This builder allows setting the user's role and permissions incrementally before building the final `UserAccess` object.
///
/// Fields:
/// - `role`: Numeric value representing the user's role.
/// - `permission`: A collection of permissions to be assigned to the user.
pub(crate) struct UserAccessBuilder {
    role: usize,
    permission: Vec<String>,
}

impl UserAccessBuilder {
    pub fn role(&mut self, role: usize) -> &mut Self {
        self.role = role;
        self
    }

    pub fn permission(&mut self, permission: Vec<String>) -> &mut Self {
        self.permission = permission;
        self
    }

    pub fn build(self) -> UserAccess {
        UserAccess {
            role: self.role,
            permission: self.permission,
        }
    }
}
