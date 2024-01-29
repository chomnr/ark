use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::{permission::manager::PermissionManager, role::manager::RoleManager},
    service::task::{
        error::TaskError,
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler,
    },
};

use super::model::User;

pub struct UserTaskHandler;

#[async_trait]
impl TaskHandler for UserTaskHandler {
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
            // pull from redis cache....
            todo!()
        }

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
    pub user: User
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, UserCreateTask> for UserCreateTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: UserCreateTask) -> TaskResponse {
        // because of how the users create their account (through oauth)
        // this operation should never fail.
        let mut pool = db.pool.get().await.unwrap();
        // dont include this as part of the transaction because if it fails the transaction fails.
        let insert_user_result = pool.query(
        "INSERT INTO iam_users (id, verified, created_at, updated_at) VALUES ($1, $2, $3, $4)",
        &[&param.user.info.user_id, &param.user.info.verified, &param.user.info.created_at, &param.user.info.updated_at]
    ).await;
        if insert_user_result.is_err() {
            match pool
                .query(
                    "SELECT u.*, r.*, p.*, o.*
            FROM iam_users u
            LEFT JOIN iam_user_role r ON u.user_id = r.user_id
            LEFT JOIN iam_user_permission p ON u.user_id = p.permission_id
            LEFT JOIN iam_user_oauth o ON u.user_id = o.user_id
            WHERE u.user_id = $1;",
                    &[&param.user.info.user_id],
                )
                .await
            {
                Ok(user) => {
                    //let user = User::new(user.get(0), user.get(1), user.get(2), user.get(3), user.get(4), user.get(5), user.get(6), user.get(7), roles, permissions, security)
                    println!("{:#?}", user.get(0));
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                Err(_) => {
                    // should be impossible...
                    return TaskResponse::throw_failed_response(
                        request,
                        vec![TaskError::UserNotFound.to_string()],
                    );
                }
            }
            // the user already exists inside the database (call the select query and return the user then cache them...)
            // check if the user exists already in the cache.
            //return Err(YourErrorType::new("User insertion failed"));
        }

        let mut transaction = pool.transaction().await.unwrap();
        transaction.execute(
        "INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)",
        &[&param.user.info.user_id, &param.user.auth.oauth_id, &param.user.auth.oauth_provider]
    ).await.unwrap();

        // Insert roles if any
        if !param.user.access.role.is_empty() {
            for role in &param.user.access.role {
                if RoleManager::get_role(&role.role_id).is_ok() {
                    transaction
                        .execute(
                            "INSERT INTO iam_user_role (user_id, role_id) VALUES ($1, $2)",
                            &[&param.user.info.user_id, &role.role_id],
                        )
                        .await
                        .unwrap();
                }
            }
        }

        if !param.user.access.permission.is_empty() {
            for permission in &param.user.access.permission {
                if PermissionManager::get_permission(&permission.permission_id).is_ok() {
                    transaction.execute(
                    "INSERT INTO iam_user_permission (user_id, permission_id) VALUES ($1, $2)",
                    &[&param.user.info.user_id, &permission.permission_id],
                ).await.unwrap();
                }
            }
        }

        transaction.commit().await.unwrap();
        return TaskResponse::compose_response(
            request,
            TaskStatus::Completed,
            param,
            Vec::default(),
        );
    }
}