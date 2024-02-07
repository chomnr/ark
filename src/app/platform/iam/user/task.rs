use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::{permission::{cache::PermissionCache, model::Permission}, role::{cache::RoleCache, model::Role}},
    service::{cache::LocalizedCache, task::{
        error::TaskError,
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler,
    }},
};

use super::{manager::UserCacheManager, model::User};

pub struct UserTaskHandler;

#[async_trait]
impl TaskHandler<PostgresDatabase> for UserTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("user_create") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserCreateTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserCreateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("user_read") {
            let payload =
                match TaskRequest::intepret_request_payload::<UserReadTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserReadTask::run(pg, task_request, payload).await;
        }

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
    pub user: User,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserCreateTask> for UserCreateTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserCreateTask,
    ) -> TaskResponse {
        // because of how the users create their account (through oauth)
        // this operation should never fail.
        let mut pool = db.pool.get().await.unwrap();
        // dont include this as part of the transaction because if it fails the transaction fails.

        let mut transaction = pool.transaction().await.unwrap();
        transaction.execute(
            "INSERT INTO iam_users (id, verified, created_at, updated_at) VALUES ($1, $2, $3, $4)",
            &[&param.user.info.user_id, &param.user.info.verified, &param.user.info.created_at, &param.user.info.updated_at]
        ).await.unwrap();

        match transaction.execute(
            "INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)",
            &[&param.user.info.user_id, &param.user.auth.oauth_id, &param.user.auth.oauth_provider]
        ).await {
            Ok(_) => {},
            Err(_) => return TaskResponse::throw_failed_response(
                request,
                vec![TaskError::UserAlreadyExists.to_string()],
            ),
        }
        if !param.user.access.role.is_empty() {
            for role_identifier in &param.user.access.role {           
                let role: Option<Role> = match RoleCache::get(role_identifier) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                if role != None {
                    transaction
                        .execute(
                            "INSERT INTO iam_user_role (user_id, role_id) VALUES ($1, $2)",
                            &[&param.user.info.user_id, &role.unwrap().role_id],
                        )
                        .await
                        .unwrap();
                }
            }
        }
        if !param.user.access.permission.is_empty() {
            for permission_identifier in &param.user.access.permission {
                let permission: Option<Permission> = match PermissionCache::get(permission_identifier) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                if permission != None {
                    transaction.execute(
                    "INSERT INTO iam_user_permission (user_id, permission_id) VALUES ($1, $2)",
                    &[&param.user.info.user_id, &permission.unwrap().permission_id],
                ).await.unwrap();
                }
            }
        }
        match transaction.commit().await {
            Ok(_) => {
                UserCacheManager::add_user_to_cache(param.user.clone()).unwrap();
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    param,
                    Vec::default(),
                )
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::UserAlreadyExists.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserReadTask {
    pub identifier: User,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserReadTask> for UserReadTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: UserReadTask,
    ) -> TaskResponse { 
        /*
        1. check if user exists in cache,
        2. retrieve if person does not exist in cache retrieve from database if user does not
        3. if user does not exist store in cache afterwards...
        user should be able to retrieved, by uid, auth_id, name or email. */
        todo!()
    }
}


// todo preload cache with user who last logged in the last 7 days.

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
