use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub log_level: String,
    pub log_path: Option<String>,
    pub oauth: OAuthConfig,
}

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_redirect_uri: String,
    pub outlook_client_id: Option<String>,
    pub outlook_client_secret: Option<String>,
    pub outlook_redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let database_url = env::var("CALENDAR_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .unwrap_or_else(|_| "calendar.db".to_string());

        let port = env::var("CALENDAR_PORT")
            .or_else(|_| env::var("PORT"))
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|e| format!("Invalid PORT: {}", e))?;

        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| "JWT_SECRET environment variable is required")?;

        if jwt_secret.is_empty() {
            return Err("JWT_SECRET must not be empty".to_string());
        }

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let log_path = env::var("LOG_PATH").ok();

        let default_base = env::var("APP_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());

        let oauth = OAuthConfig {
            google_client_id: env::var("GOOGLE_CLIENT_ID").ok(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").ok(),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .unwrap_or_else(|_| format!("{}/api/v1/connections/google/callback", default_base)),
            outlook_client_id: env::var("OUTLOOK_CLIENT_ID").ok(),
            outlook_client_secret: env::var("OUTLOOK_CLIENT_SECRET").ok(),
            outlook_redirect_uri: env::var("OUTLOOK_REDIRECT_URI")
                .unwrap_or_else(|_| format!("{}/api/v1/connections/outlook/callback", default_base)),
        };

        Ok(Config {
            database_url,
            port,
            jwt_secret,
            log_level,
            log_path,
            oauth,
        })
    }
}
