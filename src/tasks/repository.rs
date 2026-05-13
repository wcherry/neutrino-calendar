use crate::common::ApiError;
use crate::schema::{task_lists, tasks};
use crate::tasks::model::{
    NewTaskListRecord, NewTaskRecord, TaskListRecord, TaskRecord, UpdateTaskListRecord,
    UpdateTaskRecord,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct TasksRepository {
    pool: DbPool,
}

impl TasksRepository {
    pub fn new(pool: DbPool) -> Self {
        TasksRepository { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>, ApiError> {
        self.pool.get().map_err(|e| {
            tracing::error!("DB pool error: {:?}", e);
            ApiError::internal("Database connection unavailable")
        })
    }

    // ── Task Lists ────────────────────────────────────────────────────────────

    pub fn insert(&self, record: NewTaskListRecord) -> Result<TaskListRecord, ApiError> {
        let id = record.id.clone();
        let mut conn = self.get_conn()?;
        diesel::insert_into(task_lists::table)
            .values(&record)
            .execute(&mut conn)
            .map_err(|e| {
                tracing::error!("DB insert task_list error: {:?}", e);
                ApiError::internal("Database error")
            })?;
        task_lists::table
            .filter(task_lists::id.eq(&id))
            .select(TaskListRecord::as_select())
            .first(&mut conn)
            .map_err(|e| {
                tracing::error!("DB query after task_list insert error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn find_by_user(&self, user_id: &str) -> Result<Vec<TaskListRecord>, ApiError> {
        let mut conn = self.get_conn()?;
        task_lists::table
            .filter(task_lists::user_id.eq(user_id))
            .order(task_lists::name.asc())
            .select(TaskListRecord::as_select())
            .load(&mut conn)
            .map_err(|e| {
                tracing::error!("DB list task_lists error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn find_by_id(&self, id: &str, user_id: &str) -> Result<TaskListRecord, ApiError> {
        let mut conn = self.get_conn()?;
        task_lists::table
            .filter(task_lists::id.eq(id).and(task_lists::user_id.eq(user_id)))
            .select(TaskListRecord::as_select())
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::not_found("Task list not found"),
                _ => {
                    tracing::error!("DB get task_list error: {:?}", e);
                    ApiError::internal("Database error")
                }
            })
    }

    pub fn update(
        &self,
        id: &str,
        user_id: &str,
        changes: UpdateTaskListRecord,
    ) -> Result<TaskListRecord, ApiError> {
        let mut conn = self.get_conn()?;
        let affected = diesel::update(
            task_lists::table
                .filter(task_lists::id.eq(id).and(task_lists::user_id.eq(user_id))),
        )
        .set(&changes)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("DB update task_list error: {:?}", e);
            ApiError::internal("Database error")
        })?;
        if affected == 0 {
            return Err(ApiError::not_found("Task list not found"));
        }
        task_lists::table
            .filter(task_lists::id.eq(id))
            .select(TaskListRecord::as_select())
            .first(&mut conn)
            .map_err(|e| {
                tracing::error!("DB get task_list after update error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn delete(&self, id: &str, user_id: &str) -> Result<(), ApiError> {
        let mut conn = self.get_conn()?;
        let affected = diesel::delete(
            task_lists::table
                .filter(task_lists::id.eq(id).and(task_lists::user_id.eq(user_id))),
        )
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("DB delete task_list error: {:?}", e);
            ApiError::internal("Database error")
        })?;
        if affected == 0 {
            return Err(ApiError::not_found("Task list not found"));
        }
        Ok(())
    }

    // ── Tasks ─────────────────────────────────────────────────────────────────

    pub fn insert_task(&self, record: NewTaskRecord) -> Result<TaskRecord, ApiError> {
        let id = record.id.clone();
        let mut conn = self.get_conn()?;
        diesel::insert_into(tasks::table)
            .values(&record)
            .execute(&mut conn)
            .map_err(|e| {
                tracing::error!("DB insert task error: {:?}", e);
                ApiError::internal("Database error")
            })?;
        tasks::table
            .filter(tasks::id.eq(&id))
            .select(TaskRecord::as_select())
            .first(&mut conn)
            .map_err(|e| {
                tracing::error!("DB query after task insert error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn find_tasks_by_list(
        &self,
        user_id: &str,
        list_id: &str,
    ) -> Result<Vec<TaskRecord>, ApiError> {
        let mut conn = self.get_conn()?;
        tasks::table
            .filter(
                tasks::user_id
                    .eq(user_id)
                    .and(tasks::list_id.eq(list_id)),
            )
            .order((tasks::position.asc(), tasks::created_at.asc()))
            .select(TaskRecord::as_select())
            .load(&mut conn)
            .map_err(|e| {
                tracing::error!("DB list tasks error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn find_task_by_id(
        &self,
        id: &str,
        list_id: &str,
        user_id: &str,
    ) -> Result<TaskRecord, ApiError> {
        let mut conn = self.get_conn()?;
        tasks::table
            .filter(
                tasks::id
                    .eq(id)
                    .and(tasks::list_id.eq(list_id))
                    .and(tasks::user_id.eq(user_id)),
            )
            .select(TaskRecord::as_select())
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::not_found("Task not found"),
                _ => {
                    tracing::error!("DB get task error: {:?}", e);
                    ApiError::internal("Database error")
                }
            })
    }

    pub fn update_task(
        &self,
        id: &str,
        list_id: &str,
        user_id: &str,
        changes: UpdateTaskRecord,
    ) -> Result<TaskRecord, ApiError> {
        let mut conn = self.get_conn()?;
        let affected = diesel::update(
            tasks::table.filter(
                tasks::id
                    .eq(id)
                    .and(tasks::list_id.eq(list_id))
                    .and(tasks::user_id.eq(user_id)),
            ),
        )
        .set(&changes)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("DB update task error: {:?}", e);
            ApiError::internal("Database error")
        })?;
        if affected == 0 {
            return Err(ApiError::not_found("Task not found"));
        }
        tasks::table
            .filter(tasks::id.eq(id))
            .select(TaskRecord::as_select())
            .first(&mut conn)
            .map_err(|e| {
                tracing::error!("DB get task after update error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn delete_task(
        &self,
        id: &str,
        list_id: &str,
        user_id: &str,
    ) -> Result<(), ApiError> {
        let mut conn = self.get_conn()?;
        let affected = diesel::delete(
            tasks::table.filter(
                tasks::id
                    .eq(id)
                    .and(tasks::list_id.eq(list_id))
                    .and(tasks::user_id.eq(user_id)),
            ),
        )
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("DB delete task error: {:?}", e);
            ApiError::internal("Database error")
        })?;
        if affected == 0 {
            return Err(ApiError::not_found("Task not found"));
        }
        Ok(())
    }

    pub fn delete_tasks_by_list(&self, list_id: &str, user_id: &str) -> Result<(), ApiError> {
        let mut conn = self.get_conn()?;
        diesel::delete(
            tasks::table.filter(
                tasks::list_id
                    .eq(list_id)
                    .and(tasks::user_id.eq(user_id)),
            ),
        )
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("DB delete tasks by list error: {:?}", e);
            ApiError::internal("Database error")
        })?;
        Ok(())
    }
}
