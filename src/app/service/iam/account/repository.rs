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
    pub fn new() -> Self {
        Self {
            field: None,
            value: &[],
        }
    }
}

/// Builder for creating a `UserInsertion` instance.
///
/// Provides a way to construct a `UserInsertion` object incrementally, allowing optional configuration of fields and values.
///
/// Fields:
/// - `field`: Optional enum specifying the user data fields to be included in the insertion.
/// - `value`: Reference to an array of string slices representing the values for the insertion.
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
