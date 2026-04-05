use crate::common::ApiError;
use crate::events::model::{EventRecord, NewEventRecord, UpdateEventRecord};
use crate::schema::events;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct EventsRepository {
    pool: DbPool,
}

impl EventsRepository {
    pub fn new(pool: DbPool) -> Self {
        EventsRepository { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>, ApiError> {
        self.pool.get().map_err(|e| {
            tracing::error!("DB pool error: {:?}", e);
            ApiError::internal("Database connection unavailable")
        })
    }

    pub fn insert(&self, record: NewEventRecord) -> Result<EventRecord, ApiError> {
        let id = record.id.clone();
        let mut conn = self.get_conn()?;
        diesel::insert_into(events::table)
            .values(&record)
            .execute(&mut conn)
            .map_err(|e| {
                tracing::error!("DB insert event error: {:?}", e);
                ApiError::internal("Database error")
            })?;
        events::table
            .filter(events::id.eq(&id))
            .select(EventRecord::as_select())
            .first(&mut conn)
            .map_err(|e| {
                tracing::error!("DB query after event insert error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn find_by_user(
        &self,
        user_id: &str,
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
    ) -> Result<Vec<EventRecord>, ApiError> {
        let mut conn = self.get_conn()?;
        let mut query = events::table
            .filter(events::user_id.eq(user_id))
            .into_boxed();
        if let Some(f) = from {
            query = query.filter(events::start_time.ge(f));
        }
        if let Some(t) = to {
            query = query.filter(events::end_time.le(t));
        }
        query
            .order(events::start_time.asc())
            .select(EventRecord::as_select())
            .load(&mut conn)
            .map_err(|e| {
                tracing::error!("DB list events error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn find_by_id(&self, id: &str, user_id: &str) -> Result<EventRecord, ApiError> {
        let mut conn = self.get_conn()?;
        events::table
            .filter(events::id.eq(id).and(events::user_id.eq(user_id)))
            .select(EventRecord::as_select())
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::not_found("Event not found"),
                _ => {
                    tracing::error!("DB get event error: {:?}", e);
                    ApiError::internal("Database error")
                }
            })
    }

    pub fn update(
        &self,
        id: &str,
        user_id: &str,
        changes: UpdateEventRecord,
    ) -> Result<EventRecord, ApiError> {
        let mut conn = self.get_conn()?;
        let affected = diesel::update(
            events::table.filter(events::id.eq(id).and(events::user_id.eq(user_id))),
        )
        .set(&changes)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("DB update event error: {:?}", e);
            ApiError::internal("Database error")
        })?;
        if affected == 0 {
            return Err(ApiError::not_found("Event not found"));
        }
        events::table
            .filter(events::id.eq(id))
            .select(EventRecord::as_select())
            .first(&mut conn)
            .map_err(|e| {
                tracing::error!("DB get event after update error: {:?}", e);
                ApiError::internal("Database error")
            })
    }

    pub fn delete(&self, id: &str, user_id: &str) -> Result<(), ApiError> {
        let mut conn = self.get_conn()?;
        let affected =
            diesel::delete(events::table.filter(events::id.eq(id).and(events::user_id.eq(user_id))))
                .execute(&mut conn)
                .map_err(|e| {
                    tracing::error!("DB delete event error: {:?}", e);
                    ApiError::internal("Database error")
                })?;
        if affected == 0 {
            return Err(ApiError::not_found("Event not found"));
        }
        Ok(())
    }
}
