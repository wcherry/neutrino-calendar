use crate::common::{ApiError, AuthenticatedUser};
use crate::tasks::{
    dto::{
        CreateTaskListRequest, CreateTaskRequest, ListTaskListsResponse, ListTasksResponse,
        TaskListResponse, TaskResponse, UpdateTaskListRequest, UpdateTaskRequest,
    },
    model::{
        NewTaskListRecord, NewTaskRecord, UpdateTaskListRecord, UpdateTaskRecord,
    },
    repository::TasksRepository,
};
use chrono::{NaiveDateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct TasksService {
    repo: Arc<TasksRepository>,
}

impl TasksService {
    pub fn new(repo: Arc<TasksRepository>) -> Self {
        TasksService { repo }
    }

    // ── Task Lists ────────────────────────────────────────────────────────────

    pub fn list_task_lists(
        &self,
        user: &AuthenticatedUser,
    ) -> Result<ListTaskListsResponse, ApiError> {
        let records = self.repo.find_by_user(&user.user_id)?;
        let task_lists = records.into_iter().map(task_list_to_response).collect();
        Ok(ListTaskListsResponse { task_lists })
    }

    pub fn create_task_list(
        &self,
        user: &AuthenticatedUser,
        req: CreateTaskListRequest,
    ) -> Result<TaskListResponse, ApiError> {
        let now = Utc::now().naive_utc();
        let record = NewTaskListRecord {
            id: Uuid::new_v4().to_string(),
            user_id: user.user_id.clone(),
            name: req.name,
            color: req.color,
            created_at: now,
            updated_at: now,
        };
        let saved = self.repo.insert(record)?;
        Ok(task_list_to_response(saved))
    }

    pub fn get_task_list(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
    ) -> Result<TaskListResponse, ApiError> {
        let record = self.repo.find_by_id(list_id, &user.user_id)?;
        Ok(task_list_to_response(record))
    }

    pub fn update_task_list(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
        req: UpdateTaskListRequest,
    ) -> Result<TaskListResponse, ApiError> {
        let changes = UpdateTaskListRecord {
            name: req.name,
            color: req.color.map(Some),
            updated_at: Utc::now().naive_utc(),
        };
        let updated = self.repo.update(list_id, &user.user_id, changes)?;
        Ok(task_list_to_response(updated))
    }

    pub fn delete_task_list(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
    ) -> Result<(), ApiError> {
        // Verify the list belongs to the user before cascading tasks
        self.repo.find_by_id(list_id, &user.user_id)?;
        // Delete tasks first (SQLite doesn't enforce FK cascades by default)
        self.repo.delete_tasks_by_list(list_id, &user.user_id)?;
        self.repo.delete(list_id, &user.user_id)
    }

    // ── Tasks ─────────────────────────────────────────────────────────────────

    pub fn list_tasks(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
    ) -> Result<ListTasksResponse, ApiError> {
        // Verify the list belongs to the user
        self.repo.find_by_id(list_id, &user.user_id)?;
        let records = self.repo.find_tasks_by_list(&user.user_id, list_id)?;
        let tasks = records.into_iter().map(task_to_response).collect();
        Ok(ListTasksResponse { tasks })
    }

    pub fn create_task(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
        req: CreateTaskRequest,
    ) -> Result<TaskResponse, ApiError> {
        // Verify the list belongs to the user
        self.repo.find_by_id(list_id, &user.user_id)?;
        let now = Utc::now().naive_utc();
        let record = NewTaskRecord {
            id: Uuid::new_v4().to_string(),
            list_id: list_id.to_string(),
            user_id: user.user_id.clone(),
            title: req.title,
            notes: req.notes,
            done: false,
            due_date: req.due_date.as_deref().map(parse_dt).transpose()?,
            position: req.position.unwrap_or(0),
            created_at: now,
            updated_at: now,
        };
        let saved = self.repo.insert_task(record)?;
        Ok(task_to_response(saved))
    }

    pub fn get_task(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
        task_id: &str,
    ) -> Result<TaskResponse, ApiError> {
        let record = self.repo.find_task_by_id(task_id, list_id, &user.user_id)?;
        Ok(task_to_response(record))
    }

    pub fn update_task(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
        task_id: &str,
        req: UpdateTaskRequest,
    ) -> Result<TaskResponse, ApiError> {
        let changes = UpdateTaskRecord {
            title: req.title,
            notes: req.notes.map(Some),
            done: req.done,
            due_date: req.due_date.as_deref().map(parse_dt).transpose()?.map(Some),
            position: req.position,
            updated_at: Utc::now().naive_utc(),
        };
        let updated = self.repo.update_task(task_id, list_id, &user.user_id, changes)?;
        Ok(task_to_response(updated))
    }

    pub fn delete_task(
        &self,
        user: &AuthenticatedUser,
        list_id: &str,
        task_id: &str,
    ) -> Result<(), ApiError> {
        self.repo.delete_task(task_id, list_id, &user.user_id)
    }
}

fn parse_dt(s: &str) -> Result<NaiveDateTime, ApiError> {
    s.parse::<chrono::DateTime<chrono::Utc>>()
        .map(|dt| dt.naive_utc())
        .or_else(|_| s.parse::<NaiveDateTime>())
        .map_err(|_| ApiError::bad_request(&format!("Invalid datetime: {}", s)))
}

fn task_list_to_response(r: crate::tasks::model::TaskListRecord) -> TaskListResponse {
    TaskListResponse {
        id: r.id,
        name: r.name,
        color: r.color,
        created_at: r.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        updated_at: r.updated_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    }
}

fn task_to_response(r: crate::tasks::model::TaskRecord) -> TaskResponse {
    TaskResponse {
        id: r.id,
        list_id: r.list_id,
        title: r.title,
        notes: r.notes,
        done: r.done,
        due_date: r.due_date.map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        position: r.position,
        created_at: r.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        updated_at: r.updated_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    }
}
