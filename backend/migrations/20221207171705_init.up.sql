-- Add up migration script here
CREATE TABLE IF NOT EXISTS tasks (
    task_id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    task_type TEXT NOT NULL,
    state TEXT CHECK(
        state IN ('NotStarted', 'InProgress', 'Completed', 'Paused', 'Failed')
    ) NOT NULL,
    src_file TEXT NOT NULL,
    res_file TEXT
);
