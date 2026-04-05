use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Requests ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectAppleRequest {
    pub caldav_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSyncRequest {
    /// Optional connection ID to sync only one provider; omit to sync all.
    pub connection_id: Option<String>,
}

// ── Responses ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionResponse {
    pub id: String,
    pub provider: String,
    pub email: Option<String>,
    pub caldav_url: Option<String>,
    pub expires_at: Option<String>,
    pub sync_cursor: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListConnectionsResponse {
    pub connections: Vec<ConnectionResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OAuthInitResponse {
    /// Redirect the user's browser to this URL to begin the OAuth2 flow.
    pub auth_url: String,
}

// Keep old name for backwards compat with existing OpenApi references
pub use ConnectAppleRequest as ConnectRequest;
pub use ConnectionResponse as ConnectResponse;
