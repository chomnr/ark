use crate::app::service::iam::{identity::model::UserIdentity, access::model::UserAccess};

pub(crate) struct UserAccount {
    identity: UserIdentity,
    access: UserAccess
}

impl UserAccount {
    pub fn new(identity: UserIdentity, access: UserAccess) -> Self {
        Self {
            identity,
            access
        }
    }
}
