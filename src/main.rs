use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_cors::Cors;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::{RunQueryDsl, SqliteConnection};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};
use shared::init_logging;
use utoipa::{openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme}, Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod common;
mod config;
mod connections;
mod events;
mod reminder_engine;
mod reminders;
mod schema;

use crate::common::TokenService;
use crate::config::Config;
use crate::events::api::EventsApiState;
use crate::events::repository::EventsRepository;
use crate::events::service::EventsService;
use crate::reminders::api::RemindersApiState;
use crate::reminders::repository::RemindersRepository;
use crate::reminders::service::RemindersService;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

fn create_db_pool(database_url: &str) -> Result<DbPool, String> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|e| format!("Failed to create DB pool: {}", e))
}

fn run_migrations(pool: &DbPool) -> Result<(), String> {
    let mut conn = pool
        .get()
        .map_err(|e| format!("Failed to get DB connection: {}", e))?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("Failed to run migrations: {}", e))?;
    Ok(())
}

#[get("/health")]
async fn health(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => {
            error!("Health check DB connection error: {:?}", e);
            return HttpResponse::ServiceUnavailable().json(json!({
                "error": { "code": "DB_UNAVAILABLE", "message": "Database connection unavailable" }
            }));
        }
    };

    match diesel::sql_query("SELECT 1").execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
        Err(e) => {
            error!("Health check DB query error: {:?}", e);
            HttpResponse::ServiceUnavailable().json(json!({
                "error": { "code": "DB_UNHEALTHY", "message": "Database health check failed" }
            }))
        }
    }
}

use crate::events::dto::{CreateEventRequest, EventResponse, ListEventsQuery, ListEventsResponse, UpdateEventRequest};
use crate::reminders::dto::{CreateReminderRequest, ListRemindersResponse, ReminderResponse, UpdateReminderRequest};
use crate::connections::api::{ConnectRequest, ConnectResponse};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        events::api::list_events,
        events::api::create_event,
        events::api::get_event,
        events::api::update_event,
        events::api::delete_event,
        reminders::api::list_reminders,
        reminders::api::create_reminder,
        reminders::api::get_reminder,
        reminders::api::update_reminder,
        connections::api::connect_google,
        connections::api::connect_outlook,
        connections::api::connect_apple,
        connections::api::trigger_sync,
    ),
    components(
        schemas(
            CreateEventRequest, UpdateEventRequest, ListEventsQuery,
            EventResponse, ListEventsResponse,
            CreateReminderRequest, UpdateReminderRequest,
            ReminderResponse, ListRemindersResponse,
            ConnectRequest, ConnectResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "events",      description = "Calendar events"),
        (name = "reminders",   description = "Reminders and tasks"),
        (name = "connections", description = "External calendar provider connections"),
    ),
    security(("bearer_auth" = []))
)]
struct CalendarApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    });

    init_logging(&config.log_level, config.log_path.clone());

    info!("Starting Neutrino Calendar service");
    info!("Connecting to database: {}", config.database_url);

    let pool = create_db_pool(&config.database_url).unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(1);
    });

    run_migrations(&pool).unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(1);
    });

    let token_service = Arc::new(TokenService::new(config.jwt_secret.clone()));

    let events_repo = Arc::new(EventsRepository::new(pool.clone()));
    let events_service = Arc::new(EventsService::new(events_repo));
    let events_state = web::Data::new(EventsApiState { events_service });

    let reminders_repo = Arc::new(RemindersRepository::new(pool.clone()));
    let reminders_service = Arc::new(RemindersService::new(reminders_repo.clone()));
    let reminders_state = web::Data::new(RemindersApiState { reminders_service });

    // Phase 4: spawn reminder engine background worker
    let engine_repo = reminders_repo.clone();
    tokio::spawn(async move {
        reminder_engine::run(engine_repo, 60).await;
    });

    let token_service_data = web::Data::new(token_service.clone());
    let pool_data = web::Data::new(pool.clone());
    let bind_addr = format!("0.0.0.0:{}", config.port);

    info!("Listening on {}", bind_addr);

    HttpServer::new(move || {
        let openapi = CalendarApiDoc::openapi();

        App::new()
            .app_data(pool_data.clone())
            .app_data(events_state.clone())
            .app_data(reminders_state.clone())
            .app_data(token_service_data.clone())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(health)
            .service(
                web::scope("/api/v1")
                    .configure(events::api::configure)
                    .configure(reminders::api::configure)
                    .configure(connections::api::configure),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi),
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}
