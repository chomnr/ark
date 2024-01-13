use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tokio::task;

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    platform::iam::user::task::UserCreateTask,
    services::task::model::TaskType,
};

use super::model::TaskMessage;

static TASK_CHANNEL: Lazy<(Sender<TaskMessage>, Receiver<TaskMessage>)> = Lazy::new(|| unbounded());

pub struct TaskManager {
    pg: PostgresDatabase,
    redis: RedisDatabase,
}

impl TaskManager {
    pub fn with_databases(pg: PostgresDatabase, redis: RedisDatabase) -> Self {
        Self { pg, redis }
    }

    pub async fn listen(&self) {
        let pg_clone = self.pg.clone();
        task::spawn(async move {
            println!("[ARK] task initialized, now listening for incoming tasks.");
            for res in TASK_CHANNEL.1.iter() {
                println!("[ARC] received a {} task from {}", res.task_action, res.task_id);
                Self::process_task(
                    &pg_clone,
                    res.task_id,
                    res.task_action,
                    res.task_type,
                    res.task_message,
                )
                .await;
            }
        });
    }

    pub fn send(task_message: TaskMessage) {
        TASK_CHANNEL.0.send(task_message).unwrap();
    }

    async fn process_task(
        pg: &PostgresDatabase,
        task_id: String,
        task_action: String,
        task_type: TaskType,
        task_message: String,
    ) {
        let pool = pg.pool.get().await.unwrap();
        match task_type {
            TaskType::Permission => {
                //Self::process_permission_task(task_type, task_action);
            }
            TaskType::Role => {
                //Self::process_role_task(task_type, task_action);
            }
            TaskType::User => {
                Self::process_user_task(pg, task_action, task_message).await;
            }
        }
    }

    async fn process_user_task(
        pg: &PostgresDatabase,
        task_action: String,
        task_message: String,
    ) {
        let mut pool = pg.pool.get().await.unwrap();
        if task_action.eq("user_create") {
            let msg: UserCreateTask = serde_json::from_str(&task_message).unwrap();
            let transaction = pool.transaction().await.unwrap();
            transaction
                .execute(
                    &msg.sql_1,
                    &[
                        &msg.param.info.user_id,
                        &msg.param.info.username,
                        &msg.param.info.email,
                        &msg.param.info.created_at,
                        &msg.param.info.updated_at,
                    ],
                )
                .await
                .unwrap();
            transaction
                .execute(
                    &msg.sql_2,
                    &[
                        &msg.param.info.user_id,
                        &msg.param.auth.oauth_id,
                        &msg.param.auth.oauth_provider,
                    ],
                )
                .await
                .unwrap();
            transaction.commit().await.unwrap();
        }
    }

    /*
    fn process_permission_task(task_type: TaskType, task_action: String) {}

    fn process_role_task(task_type: TaskType, task_action: String) {}

    fn process_user_task(task_type: TaskType, task_action: String) {
        todo!()
    }
    */
}

// Clone the necessary data from `self`
//let pg_clone = self.pg.clone();
//let pool = pg_clone.pool.get().await.unwrap();
//pub fn send(task_message: TaskMessage) {
//  TASK_CHANNEL.0.send(task_message).unwrap();
//}

/*
async fn process_query(
    &self,
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<u64, Error> {
    let pool = self.pg.pool.get().await.unwrap();
    let stmt = pool.prepare(query).await.unwrap();
    pool.execute(&stmt, params).await
}
*/

/*
if message.task_action.eq("user_create_task") {
    //UserTaskManager::perform("user_create_task");
    // create user here test...
    //let task_create: UserCreateTask = serde_json::from_str(&message.task_message).unwrap();
    //UserTaskManager::perform("user_create_task");
    // perform query here...
    //println!("{}", task_create.param.info.username);
}
*/
