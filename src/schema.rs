// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Text,
        user_id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        all_day -> Bool,
        location -> Nullable<Text>,
        recurrence_rule -> Nullable<Text>,
        external_id -> Nullable<Text>,
        source -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reminders (id) {
        id -> Text,
        user_id -> Text,
        title -> Text,
        due_time -> Timestamp,
        completed -> Bool,
        recurrence_rule -> Nullable<Text>,
        linked_event_id -> Nullable<Text>,
        notified_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    event_attachments (id) {
        id -> Text,
        event_id -> Text,
        file_id -> Text,
    }
}

diesel::table! {
    calendar_connections (id) {
        id -> Text,
        user_id -> Text,
        provider -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamp>,
        sync_cursor -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
