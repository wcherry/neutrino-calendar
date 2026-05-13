CREATE TABLE tasks (
    id         TEXT PRIMARY KEY NOT NULL,
    user_id    TEXT NOT NULL,
    title      TEXT NOT NULL,
    notes      TEXT,
    done       BOOLEAN NOT NULL DEFAULT 0,
    due_date   TIMESTAMP,
    position   INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tasks_user ON tasks (user_id);

CREATE TABLE task_list_memberships (
    task_id  TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    list_id  TEXT NOT NULL REFERENCES task_lists(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, list_id)
);

CREATE INDEX idx_memberships_task ON task_list_memberships (task_id);
CREATE INDEX idx_memberships_list ON task_list_memberships (list_id);
