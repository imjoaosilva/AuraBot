-- Add migration script here
CREATE TABLE IF NOT EXISTS channels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    logs_channel_id INTEGER NOT NULL,
    meta_channel_id INTEGER NOT NULL,
    anonymous_channel_id INTEGER NOT NULL,
    individuals_category_id INTEGER NOT NULL,
    approval_channel_id INTEGER NOT NULL
);
