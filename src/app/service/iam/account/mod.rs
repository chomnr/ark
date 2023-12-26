pub mod model;

//let test = IdentityRepository::new(postgres_db);
//test.create_identity(identity);

//let test2 = AccessRepository::new(postgres_db);
//test2.add_permission(identity, "ban.account");

//let test3 = SessionRepository::new(redis_db)
//test3.create_session("ddsdadasdas, dsaadsasd") checks in cache

//let test4 = IamRepository::new(postgres_db)
//test4.create_identity("dasdsad")


/*
static CREATE_IDENTITY_QUERY: &str ="INSERT INTO identity (username, email, oauth_provider, oauth_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (oauth_id)
            DO UPDATE SET last_login = CURRENT_TIMESTAMP
            RETURNING *;";


INSERT INTO identity (username, email, oauth_provider, oauth_id)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (oauth_id)
                DO UPDATE SET last_login = CURRENT_TIMESTAMP
                RETURNING *;"

/// Inserts a user into identity.
static CREATE_IDENTITY_QUERY: &str =
    "INSERT INTO identity (username, email, oauth_provider, oauth_id)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (oauth_id)
                DO UPDATE SET last_login = CURRENT_TIMESTAMP
                RETURNING *;";

/// A repository responsible for managing user data storage and retrieval.
///
/// This struct encapsulates the operations and mechanisms needed to interact with the storage layer for user accounts.
/// It typically includes methods for creating, updating, retrieving, and deleting user records.
pub(crate) struct UserRepository;

impl UserRepository {
    // insert stuff.
    pub fn insert_mode<'a>(account: UserAccount) -> UserInsertionBuilder<'static> {
        UserInsertion::new(account)
    }
}

/// Enumerates the fields used for inserting user data into the `UserRepository`.
///
/// Variants:
/// - `All`: Indicates that all user-related fields should be included in the insertion.
#[derive(PartialEq)]
pub(crate) enum UserInsertionField {
    Identity,
    Permission,
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
    fn new(account: UserAccount) -> UserInsertionBuilder<'static> {
        UserInsertionBuilder {
            account,
            field: UserInsertionField::Identity,
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
    pub fn field(&mut self, field: UserInsertionField) -> &mut Self {
        self.field = field;
        self
    }

    pub fn value<'a>(&mut self, value: &'static [&'a str]) -> &mut Self {
        self.value = value;
        self
    }

    pub async fn execute_on(&self, pg: PostgresDatabase) -> AccountRepositoryResult<()> {
        let client = pg.get().await;
        // identity
        if self.field.eq(&UserInsertionField::Identity) && self.value.len() == 0 {
            let statement = client.prepare(CREATE_IDENTITY_QUERY).await;
            client
                .execute(&statement.unwrap(), &[&"1", &"2", &"3", &"4"])
                .await
                .map_err(|e| AccountRepositoryError::FailedToCreateIdentity)?;
        }
        // permission
        if self.field.eq(&UserInsertionField::Permission) && self.value.len() > 0 {
            // test 2
            // todo with permission ensure i update the cache. (trigger the event somehow.)
        }
        Err(AccountRepositoryError::FieldMismatch)
    }
}
*/
