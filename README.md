# neutrino-calendar

Calendar service for the Neutrino suite. Manages events and reminders with support for syncing external calendars from Google, Outlook, and Apple (CalDAV).

## Features

- Create, read, update, and delete calendar events
- Reminders with due dates and recurrence (RRULE)
- OAuth2 integration with Google Calendar and Outlook
- Apple CalDAV connection support
- Background reminder engine (polls every 60s)
- JWT-authenticated REST API with Swagger UI

## Tech Stack

- **Language:** Rust (edition 2021)
- **Framework:** Actix-web 4
- **Database:** SQLite via Diesel 2 ORM
- **Auth:** JWT (jsonwebtoken)
- **API Docs:** utoipa / Swagger UI

## Getting Started

### Prerequisites

- Rust 1.93+

### Run locally

```bash
export JWT_SECRET="your-secret-key"
export LOG_LEVEL=debug

cargo run
```

The server starts on `0.0.0.0:8080`. Swagger UI is available at `http://localhost:8080/swagger-ui/`.

## Configuration

| Variable | Required | Default | Description |
|---|---|---|---|
| `JWT_SECRET` | Yes | — | JWT signing secret |
| `CALENDAR_DATABASE_URL` / `DATABASE_URL` | No | `calendar.db` | SQLite file path |
| `CALENDAR_PORT` / `PORT` | No | `8080` | HTTP listen port |
| `LOG_LEVEL` | No | `info` | `debug`, `info`, `warn`, `error` |
| `LOG_PATH` | No | — | Directory for log files |
| `APP_BASE_URL` | No | `http://localhost:8080` | Base URL for OAuth redirects |
| `GOOGLE_CLIENT_ID` | No | — | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | No | — | Google OAuth client secret |
| `GOOGLE_REDIRECT_URI` | No | `{APP_BASE_URL}/api/v1/connections/google/callback` | Google OAuth callback |
| `OUTLOOK_CLIENT_ID` | No | — | Outlook OAuth client ID |
| `OUTLOOK_CLIENT_SECRET` | No | — | Outlook OAuth client secret |
| `OUTLOOK_REDIRECT_URI` | No | `{APP_BASE_URL}/api/v1/connections/outlook/callback` | Outlook OAuth callback |

## API

All endpoints (except `/health`) require a `Bearer` JWT token in the `Authorization` header.

### Events

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/events` | List events (`from`, `to` query params, ISO 8601 UTC) |
| `POST` | `/api/v1/events` | Create event |
| `GET` | `/api/v1/events/{id}` | Get event |
| `PUT` | `/api/v1/events/{id}` | Update event |
| `DELETE` | `/api/v1/events/{id}` | Delete event |

### Reminders

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/reminders` | List reminders |
| `POST` | `/api/v1/reminders` | Create reminder |
| `GET` | `/api/v1/reminders/{id}` | Get reminder |
| `PATCH` | `/api/v1/reminders/{id}` | Update reminder |
| `DELETE` | `/api/v1/reminders/{id}` | Delete reminder |

### Connections

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/connections` | List connected providers |
| `POST` | `/api/v1/connections/google` | Start Google OAuth flow |
| `GET` | `/api/v1/connections/google/callback` | Google OAuth callback |
| `POST` | `/api/v1/connections/outlook` | Start Outlook OAuth flow |
| `GET` | `/api/v1/connections/outlook/callback` | Outlook OAuth callback |
| `POST` | `/api/v1/connections/apple` | Connect Apple CalDAV |
| `DELETE` | `/api/v1/connections/{id}` | Disconnect provider |
| `POST` | `/api/v1/sync/trigger` | Trigger manual sync (optional `connection_id`) |

### Other

| Method | Path | Description |
|---|---|---|
| `GET` | `/health` | Health check |
| `GET` | `/swagger-ui/{path}` | Swagger UI |
| `GET` | `/api-docs/openapi.json` | OpenAPI spec |

## Docker

```bash
docker build -t neutrino-calendar .

docker run \
  -e JWT_SECRET="secret" \
  -v /path/to/data:/usr/local/data \
  -v /path/to/logs:/usr/local/logs \
  -p 8080:8080 \
  neutrino-calendar
```

In production, the service expects secrets at `/run/secrets/jwt_secret`, data at `/usr/local/data/`, and writes logs to `/usr/local/logs/`.

See the root `docker-compose-dev.yml` for the full Neutrino stack configuration.

## Project Structure

```
src/
├── main.rs               # Server setup and route registration
├── schema.rs             # Diesel schema
├── common/               # JWT auth extractor, error types, token utils
├── config/               # Environment variable loading
├── events/               # Event CRUD (api, service, repository, models)
├── reminders/            # Reminder management
├── connections/          # External provider integrations
│   ├── google.rs         # Google OAuth2 and Calendar API
│   ├── outlook.rs        # Outlook OAuth2
│   └── apple.rs          # Apple CalDAV client
└── reminder_engine/      # Background task: polls and fires due reminders
migrations/               # Diesel SQL migrations
```
