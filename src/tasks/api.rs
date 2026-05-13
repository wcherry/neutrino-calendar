use crate::common::{ApiError, AuthenticatedUser};
use crate::tasks::{
    dto::{
        CreateTaskListRequest, CreateTaskRequest, ListTaskListsResponse, ListTasksResponse,
        TaskListResponse, TaskResponse, UpdateTaskListRequest, UpdateTaskRequest,
    },
    service::TasksService,
};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use std::sync::Arc;
use utoipa::OpenApi;

pub struct TasksApiState {
    pub tasks_service: Arc<TasksService>,
}

// ── Task Lists ────────────────────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/task-lists",
    responses(
        (status = 200, description = "List of task lists", body = ListTaskListsResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[get("/task-lists")]
pub async fn list_task_lists(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
) -> Result<web::Json<ListTaskListsResponse>, ApiError> {
    let result = state.tasks_service.list_task_lists(&user)?;
    Ok(web::Json(result))
}

#[utoipa::path(
    post,
    path = "/api/v1/task-lists",
    request_body = CreateTaskListRequest,
    responses(
        (status = 201, description = "Task list created", body = TaskListResponse),
        (status = 400, description = "Invalid request"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[post("/task-lists")]
pub async fn create_task_list(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    body: web::Json<CreateTaskListRequest>,
) -> Result<HttpResponse, ApiError> {
    let list = state
        .tasks_service
        .create_task_list(&user, body.into_inner())?;
    Ok(HttpResponse::Created().json(list))
}

#[utoipa::path(
    get,
    path = "/api/v1/task-lists/{id}",
    params(("id" = String, Path, description = "Task list ID")),
    responses(
        (status = 200, description = "Task list", body = TaskListResponse),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[get("/task-lists/{id}")]
pub async fn get_task_list(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<web::Json<TaskListResponse>, ApiError> {
    let list = state
        .tasks_service
        .get_task_list(&user, &path.into_inner())?;
    Ok(web::Json(list))
}

#[utoipa::path(
    patch,
    path = "/api/v1/task-lists/{id}",
    params(("id" = String, Path, description = "Task list ID")),
    request_body = UpdateTaskListRequest,
    responses(
        (status = 200, description = "Task list updated", body = TaskListResponse),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[patch("/task-lists/{id}")]
pub async fn update_task_list(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<String>,
    body: web::Json<UpdateTaskListRequest>,
) -> Result<web::Json<TaskListResponse>, ApiError> {
    let list = state
        .tasks_service
        .update_task_list(&user, &path.into_inner(), body.into_inner())?;
    Ok(web::Json(list))
}

#[utoipa::path(
    delete,
    path = "/api/v1/task-lists/{id}",
    params(("id" = String, Path, description = "Task list ID")),
    responses(
        (status = 204, description = "Task list deleted"),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[delete("/task-lists/{id}")]
pub async fn delete_task_list(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    state
        .tasks_service
        .delete_task_list(&user, &path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}

// ── Tasks ─────────────────────────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/task-lists/{list_id}/tasks",
    params(("list_id" = String, Path, description = "Task list ID")),
    responses(
        (status = 200, description = "List of tasks", body = ListTasksResponse),
        (status = 404, description = "List not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[get("/task-lists/{list_id}/tasks")]
pub async fn list_tasks(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<web::Json<ListTasksResponse>, ApiError> {
    let result = state
        .tasks_service
        .list_tasks(&user, &path.into_inner())?;
    Ok(web::Json(result))
}

#[utoipa::path(
    post,
    path = "/api/v1/task-lists/{list_id}/tasks",
    params(("list_id" = String, Path, description = "Task list ID")),
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created", body = TaskResponse),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "List not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[post("/task-lists/{list_id}/tasks")]
pub async fn create_task(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<String>,
    body: web::Json<CreateTaskRequest>,
) -> Result<HttpResponse, ApiError> {
    let task = state
        .tasks_service
        .create_task(&user, &path.into_inner(), body.into_inner())?;
    Ok(HttpResponse::Created().json(task))
}

#[utoipa::path(
    get,
    path = "/api/v1/task-lists/{list_id}/tasks/{id}",
    params(
        ("list_id" = String, Path, description = "Task list ID"),
        ("id" = String, Path, description = "Task ID"),
    ),
    responses(
        (status = 200, description = "Task", body = TaskResponse),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[get("/task-lists/{list_id}/tasks/{id}")]
pub async fn get_task(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<(String, String)>,
) -> Result<web::Json<TaskResponse>, ApiError> {
    let (list_id, task_id) = path.into_inner();
    let task = state.tasks_service.get_task(&user, &list_id, &task_id)?;
    Ok(web::Json(task))
}

#[utoipa::path(
    patch,
    path = "/api/v1/task-lists/{list_id}/tasks/{id}",
    params(
        ("list_id" = String, Path, description = "Task list ID"),
        ("id" = String, Path, description = "Task ID"),
    ),
    request_body = UpdateTaskRequest,
    responses(
        (status = 200, description = "Task updated", body = TaskResponse),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[patch("/task-lists/{list_id}/tasks/{id}")]
pub async fn update_task(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<(String, String)>,
    body: web::Json<UpdateTaskRequest>,
) -> Result<web::Json<TaskResponse>, ApiError> {
    let (list_id, task_id) = path.into_inner();
    let task = state
        .tasks_service
        .update_task(&user, &list_id, &task_id, body.into_inner())?;
    Ok(web::Json(task))
}

#[utoipa::path(
    delete,
    path = "/api/v1/task-lists/{list_id}/tasks/{id}",
    params(
        ("list_id" = String, Path, description = "Task list ID"),
        ("id" = String, Path, description = "Task ID"),
    ),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "tasks"
)]
#[delete("/task-lists/{list_id}/tasks/{id}")]
pub async fn delete_task(
    state: web::Data<TasksApiState>,
    user: AuthenticatedUser,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let (list_id, task_id) = path.into_inner();
    state.tasks_service.delete_task(&user, &list_id, &task_id)?;
    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(list_task_lists)
        .service(create_task_list)
        .service(get_task_list)
        .service(update_task_list)
        .service(delete_task_list)
        .service(list_tasks)
        .service(create_task)
        .service(get_task)
        .service(update_task)
        .service(delete_task);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        list_task_lists,
        create_task_list,
        get_task_list,
        update_task_list,
        delete_task_list,
        list_tasks,
        create_task,
        get_task,
        update_task,
        delete_task,
    ),
    components(schemas(
        CreateTaskListRequest,
        UpdateTaskListRequest,
        TaskListResponse,
        ListTaskListsResponse,
        CreateTaskRequest,
        UpdateTaskRequest,
        TaskResponse,
        ListTasksResponse,
    )),
    tags((name = "tasks", description = "Task lists and tasks")),
    security(("bearer_auth" = []))
)]
pub struct TasksApiDoc;
