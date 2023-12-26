/*
use crate::app::service::iam::{identity::model::UserIdentity, access::model::UserAccess};

pub(crate) struct User {
    identity: UserIdentity,
    access: UserAccess
}

impl User {
    pub fn new(identity: UserIdentity, access: UserAccess) -> Self {
        Self {
            identity,
            access,
        }
    }
}
*/

/*
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
*/