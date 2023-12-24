use crate::app::service::iam::identity::model::{UserIdentity, UserIdentityBuilder};

use super::model::UserAccount;

/// Enumerates the fields used for inserting user data into the `UserRepository`.
///
/// Variants:
/// - `All`: Indicates that all user-related fields should be included in the insertion.
pub(crate) enum UserInsertionField {
    All,
}

/// A repository responsible for managing user data storage and retrieval.
///
/// This struct encapsulates the operations and mechanisms needed to interact with the storage layer for user accounts.
/// It typically includes methods for creating, updating, retrieving, and deleting user records.
pub(crate) struct UserRepository;

impl UserRepository {
    // insert stuff.
    pub fn insert<'a>(account: UserAccount) -> UserInsertion<'a> {
        UserInsertion::new()
    }
}

pub(crate) struct UserInsertion<'a> {
    field: Option<UserInsertionField>,
    value: &'a [&'a str],
}

impl UserInsertion<'_> {
    pub fn new() -> Self {
        Self {
            field: None,
            value: &[],
        }
    }
}

pub(crate) struct UserInsertionBuilder<'a> {
    field: Option<UserInsertionField>,
    value: &'a [&'a str],
}

impl UserInsertionBuilder<'_> {

}


/*
pub(crate) struct UserInsertionBuilder;

impl UserInsertionBuilder {
    pub fn modify(self, field: UserInsertionField) {}
}
*/
