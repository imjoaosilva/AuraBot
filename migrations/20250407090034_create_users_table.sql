-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,  
    discord_id TEXT UNIQUE NOT NULL,             
    channel_id TEXT NOT NULL,             
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
