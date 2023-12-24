use crate::app::database::postgres::PostgresDatabase;

use super::model::UserAccount;

/// Enumerates the fields used for inserting user data into the `UserRepository`.
///
/// Variants:
/// - `All`: Indicates that all user-related fields should be included in the insertion.
#[derive(PartialEq)]
pub(crate) enum UserInsertionField {
    All,
    Permission
}

/// A repository responsible for managing user data storage and retrieval.
///
/// This struct encapsulates the operations and mechanisms needed to interact with the storage layer for user accounts.
/// It typically includes methods for creating, updating, retrieving, and deleting user records.
pub(crate) struct UserRepository;

impl UserRepository {
    // insert stuff.
    pub fn insert<'a>(account: UserAccount) -> UserInsertionBuilder<'static> {
        UserInsertion::new(account)
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
    account: UserAccount,
    field: UserInsertionField,
    value: &'a [&'a str],
}

impl UserInsertion<'_> {
    pub fn new(account: UserAccount) -> UserInsertionBuilder<'static> {
        UserInsertionBuilder {
            account,
            field: UserInsertionField::All,
            value: &[],
        }
    }
}

pub(crate) struct UserInsertionBuilder<'a> {
    account: UserAccount,
    field: UserInsertionField,
    value: &'a [&'a str],
}

impl UserInsertionBuilder<'_> {
    pub fn modify(&mut self, field: UserInsertionField) -> &mut Self {
        self.field = field;
        self
    }

    pub fn value<'a>(&mut self, value: &'static[&'a str]) -> &mut Self {
        self.value = value;
        self
    }

    pub async fn execute_on(&self, pg: PostgresDatabase) {
        if self.field.eq(&UserInsertionField::All) && self.value.len() == 0 {
            // insert stuff here...
        }

        if self.field.eq(&UserInsertionField::Permission) && self.value.len() > 0 {
            // insert stuff here...
        }
    }
}

/*
pub fn value<'a>(&mut self, value: &'static[&'a str]) -> &mut Self {
    self.value = value;
    self
}
*/