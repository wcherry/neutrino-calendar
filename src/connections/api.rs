use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectRequest {
    /// OAuth authorization code from the provider.
    pub code: String,
    /// Redirect URI used during the OAuth flow.
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResponse {
    pub provider: String,
    pub connected: bool,
}

/// Connect a Google Calendar account (Phase 3).
#[utoipa::path(
    post,
    path = "/api/v1/connections/google",
    request_body = ConnectRequest,
    responses(
        (status = 200, description = "Connected", body = ConnectResponse),
        (status = 501, description = "Not yet implemented"),
    ),
    security(("bearer_auth" = [])),
    tag = "connections"
)]
#[post("/connections/google")]
pub async fn connect_google(_body: web::Json<ConnectRequest>) -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": { "code": "NOT_IMPLEMENTED", "message": "Google Calendar sync coming in Phase 3" }
    }))
}

/// Connect an Outlook calendar account (Phase 3).
#[utoipa::path(
    post,
    path = "/api/v1/connections/outlook",
    request_body = ConnectRequest,
    responses(
        (status = 200, description = "Connected", body = ConnectResponse),
        (status = 501, description = "Not yet implemented"),
    ),
    security(("bearer_auth" = [])),
    tag = "connections"
)]
#[post("/connections/outlook")]
pub async fn connect_outlook(_body: web::Json<ConnectRequest>) -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": { "code": "NOT_IMPLEMENTED", "message": "Outlook calendar sync coming in Phase 3" }
    }))
}

/// Connect an Apple / iCloud calendar account via CalDAV (Phase 3).
#[utoipa::path(
    post,
    path = "/api/v1/connections/apple",
    request_body = ConnectRequest,
    responses(
        (status = 200, description = "Connected", body = ConnectResponse),
        (status = 501, description = "Not yet implemented"),
    ),
    security(("bearer_auth" = [])),
    tag = "connections"
)]
#[post("/connections/apple")]
pub async fn connect_apple(_body: web::Json<ConnectRequest>) -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": { "code": "NOT_IMPLEMENTED", "message": "Apple CalDAV sync coming in Phase 3" }
    }))
}

/// Trigger an immediate sync for all connected providers (Phase 3).
#[utoipa::path(
    post,
    path = "/api/v1/sync/trigger",
    responses(
        (status = 202, description = "Sync enqueued"),
        (status = 501, description = "Not yet implemented"),
    ),
    security(("bearer_auth" = [])),
    tag = "connections"
)]
#[post("/sync/trigger")]
pub async fn trigger_sync() -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": { "code": "NOT_IMPLEMENTED", "message": "External sync coming in Phase 3" }
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(connect_google)
        .service(connect_outlook)
        .service(connect_apple)
        .service(trigger_sync);
}

#[derive(OpenApi)]
#[openapi(
    paths(connect_google, connect_outlook, connect_apple, trigger_sync),
    components(schemas(ConnectRequest, ConnectResponse)),
    tags((name = "connections", description = "External calendar provider connections")),
    security(("bearer_auth" = []))
)]
pub struct ConnectionsApiDoc;
