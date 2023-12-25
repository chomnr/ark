use crate::app::service::iam::{identity::model::UserIdentity, access::model::UserAccess};
// SessionHandler::trigger(SessionEvent::UPDATE_CACHE, identity, identity)

/// Represents a user's account, encompassing identity and access control details.
///
/// Fields:
/// - `identity`: A `UserIdentity` struct containing the user's identity information such as username and email.
/// - `access`: A `UserAccess` struct detailing the user's access level and permissions.
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
