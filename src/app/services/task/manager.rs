use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tokio::task;

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    services::task::model::TaskType,
};

use super::{model::TaskMessage, error::TaskResult};

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
        task::spawn(async move {
            println!("[ARK] task initialized, now listening for incoming tasks.");
            for res in TASK_CHANNEL.1.iter() {
                Self::process_task(
                    res.task_id,
                    res.task_action,
                    res.task_type,
                    res.task_message,
                );
            }
        });
    }

    fn process_task(
        task_id: String,
        task_action: String,
        task_type: TaskType,
        task_message: String,
    ) {
        match task_type {
            TaskType::Permission => {
                Self::process_permission_task(task_type, task_action);
            },
            TaskType::Role => {
                Self::process_role_task(task_type, task_action);
            },
            TaskType::User => {
                Self::process_user_task(task_type, task_action).expect("failed ot process task...");
            }
        }
    }

    fn process_permission_task(task_type: TaskType, task_action: String) {}

    fn process_role_task(task_type: TaskType, task_action: String) {}

    fn process_user_task(task_type: TaskType, task_action: String) -> TaskResult<bool> {
        todo!()
    }
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
