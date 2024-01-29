use axum::async_trait;

use crate::app::{service::task::{TaskHandler, message::{TaskRequest, TaskResponse}, error::TaskError}, database::postgres::PostgresDatabase};

pub struct UserTaskHandler;

#[async_trait]
impl TaskHandler for UserTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("user_create") {
            todo!()
        }

        if task_request.task_action.eq("user_read") {
            // pull from redis cache....
            todo!()
        }
        
        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

/*
/// Represents a task for creating a new user, containing SQL statements and user parameters.
#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
    sql_1: String,
    sql_2: String,
    sql_3: String,
    sql_4: String,
    pub param: User,
}

impl Default for UserCreateTask {
    fn default() -> Self {
        Self {
            sql_1: String::from("INSERT INTO iam_users (id, username, email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"),
            sql_2: String::from("INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)"),
            sql_3: String::from("INSERT INTO iam_roles (id, role_name) VALUES ($1, $2)"),
            sql_4: String::from("todo"),
            param: Default::default(),
        }
    }
}

impl UserCreateTask {
    pub fn new() -> Self {
        UserCreateTask::default()
    }

    pub async fn process(&self, pg: &PostgresDatabase) -> TaskResult<()> {
        let mut pool = pg.pool.get().await.unwrap();
        let mut transaction = pool.transaction().await.unwrap();
        // UserCreateTask Here...........
        match transaction.commit().await {
            Ok(_) => Ok(()),
            Err(_) => Err(TaskError::TaskWentWrong), // 
        }
        // check if user is in cache
        // check if user exists.
        // then process transaction.
    }
}
*/