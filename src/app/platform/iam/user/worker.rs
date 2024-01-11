use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum UserWorkerAction {
    CreateUser
}

#[derive(Serialize, Deserialize)]
pub struct UserWorkerMessage {
    pub message: String
}

//static TASK_REQUESTS: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(|| unbounded::<String>());
//static TASK_RESULTS: Lazy<(Sender<String>, Receiver<String>)> = Lazy::new(|| unbounded::<String>());


/* 
static TASK_SENDER: Lazy<Sender<UserWorkerTask>> = Lazy::new(|| {
    let (sender, _) = mpsc::channel(1000);
    sender
});

static TASK_RECEIVER: Lazy<RwLock<Receiver<UserWorkerTask>>> = Lazy::new(|| {
    let (_, receiver) = mpsc::channel(1000);
    RwLock::new(receiver)
});

struct UserWorkerTask {
    task_id: String
}

pub struct UserWorker {
    pg: PostgresDatabase
}

impl UserWorker {
    pub async fn new_worker(pg: PostgresDatabase) -> UserWorker {
        let worker = UserWorker { pg };
    
        task::spawn(async move {
            loop {
                let receiver = {
                    let lock = TASK_RECEIVER.read().await;
                    lock
                };
    
                match receiver.recv().await {
                    Ok(Some(task)) => {
                        // Process the task
                        // ...
                    },
                    Ok(None) => {
                        // The channel has been closed and all senders have been dropped
                        break;
                    },
                    Err(_) => {
                        // Handle error (if applicable)
                        break;
                    }
                }
            }
        });
    
        worker
    }

    pub fn add_task(task: UserWorkerTask) {
        TASK_SENDER.send(task);
    }
}


pub fn test() {
    
}
*/

/*
static TASKS: Lazy<RwLock<LinkedList<UserWorkerTask>>> = Lazy::new(|| {
    RwLock::new(LinkedList::new())
});

struct UserWorkerTask<'a> {
    task_id: String,
    sql: Option<String>,
    parameters: Option<&'a [&'a str]>,
}

pub struct UserWorker {
    pg: PostgresDatabase
}

impl UserWorker {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self {
            pg,
        }
    }

    pub async fn listen(&self) {
        
    }
}
*/
