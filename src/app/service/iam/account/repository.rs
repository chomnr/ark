use crate::app::service::iam::identity::model::UserIdentityBuilder;

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
    pub fn insert<'a>(account: UserAccount) -> UserInsertionBuilder<'static> {
        UserInsertion::new()
    }
}

/// Represents the data for inserting a user record.
///
/// This struct is used to specify which fields and corresponding values to include when inserting a user record into a repository.
///
/// Fields:
/// - `field`: Optional enum specifying which fields of the user data to insert (`UserInsertionField`).
/// - `value`: Reference to an array of string slices representing the values to be inserted.
pub(crate) struct UserInsertion<'a> {
    field: Option<UserInsertionField>,
    value: &'a [&'a str],
}

impl UserInsertion<'_> {
    pub fn new() -> UserInsertionBuilder<'static> {
        UserInsertionBuilder {
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
    pub fn modify(&mut self, field: UserInsertionField) -> &mut Self {
        self.field = Some(field);
        self
    }
}


/*
pub(crate) struct UserInsertionBuilder;

impl UserInsertionBuilder {
    pub fn modify(self, field: UserInsertionField) {}
}
*/
