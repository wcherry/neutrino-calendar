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
        timezone -> Nullable<Text>,
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
        file_id -> Nullable<Text>,
        name -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    event_attendees (id) {
        id -> Text,
        event_id -> Text,
        email -> Text,
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
        email -> Nullable<Text>,
        caldav_url -> Nullable<Text>,
    }
}

diesel::table! {
    task_lists (id) {
        id -> Text,
        user_id -> Text,
        name -> Text,
        color -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        user_id -> Text,
        title -> Text,
        notes -> Nullable<Text>,
        done -> Bool,
        due_date -> Nullable<Timestamp>,
        position -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    task_list_memberships (task_id, list_id) {
        task_id -> Text,
        list_id -> Text,
    }
}

diesel::joinable!(task_list_memberships -> tasks (task_id));
diesel::joinable!(task_list_memberships -> task_lists (list_id));

diesel::allow_tables_to_appear_in_same_query!(
    task_list_memberships,
    tasks,
    task_lists,
);
