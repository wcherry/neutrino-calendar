CREATE TABLE tasks (
    id         TEXT PRIMARY KEY NOT NULL,
    list_id    TEXT NOT NULL REFERENCES task_lists(id),
    user_id    TEXT NOT NULL,
    title      TEXT NOT NULL,
    notes      TEXT,
    done       BOOLEAN NOT NULL DEFAULT 0,
    due_date   TIMESTAMP,
    position   INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tasks_user_list ON tasks (user_id, list_id);
CREATE INDEX idx_tasks_list_position ON tasks (list_id, position);
