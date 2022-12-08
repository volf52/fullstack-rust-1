use std::str::FromStr;

use common::models::task::{Task, TaskState};
use sqlx::Sqlite;

use crate::db::POOL;

struct TaskStateDb(TaskState);

#[derive(sqlx::Encode, sqlx::Decode, sqlx::Type, sqlx::FromRow)]
struct TaskDb {
    pub user_id: String,
    pub task_id: String,
    pub task_type: String,
    pub state: String,
    pub src_file: String,
    pub res_file: Option<String>,
}

impl From<&Task> for TaskDb {
    fn from(_: &Task) -> Self {
        todo!()
    }
}

impl From<TaskDb> for Task {
    fn from(val: TaskDb) -> Self {
        let state = TaskState::from_str(&val.state).unwrap();
        Task {
            state,
            user_id: val.user_id,
            task_id: val.task_id,
            src_file: val.src_file,
            res_file: val.res_file,
            task_type: val.task_type,
        }
    }
}

impl<'q> sqlx::Encode<'q, Sqlite> for TaskStateDb {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        sqlx::Encode::<Sqlite>::encode(self.0.to_string(), buf)
    }
}

impl<'r> sqlx::Decode<'r, Sqlite> for TaskStateDb {
    fn decode(
        value: <Sqlite as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let v = <String as sqlx::Decode<Sqlite>>::decode(value)?;
        let state = TaskState::from_str(&v)?;

        Ok(TaskStateDb(state))
    }
}

impl sqlx::Type<Sqlite> for TaskStateDb {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<Sqlite>>::type_info()
    }
}

pub struct TaskRepo;

impl TaskRepo {
    pub async fn put_task(task: &Task) {
        let pool = POOL.get().expect("Failed to get POOL");
        let task_state_db = TaskStateDb(task.state.clone());

        sqlx::query!("INSERT INTO 'tasks' (task_id, user_id, task_type, state, src_file, res_file) VALUES ($1, $2, $3, $4, $5, $6)", task.task_id, task.user_id, task.task_type, task_state_db, task.src_file, task.res_file).execute(pool).await.unwrap();
    }

    pub async fn get_task(task_id: String) -> Option<Task> {
        let pool = POOL.get().expect("Failed to get POOL");

        let task = sqlx::query_as!(TaskDb, "SELECT * from tasks where task_id = $1", task_id)
            .fetch_optional(pool)
            .await
            .unwrap();

        task.map(|task| task.into())
    }

    pub async fn get_all_tasks() -> Vec<Task> {
        let pool = POOL.get().expect("Failed to get POOL");

        let tasks = sqlx::query_as!(TaskDb, "SELECT * from tasks")
            .fetch_all(pool)
            .await
            .unwrap();

        tasks.into_iter().map(|t| t.into()).collect()
    }
}
