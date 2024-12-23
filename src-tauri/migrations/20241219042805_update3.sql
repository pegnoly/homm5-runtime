-- Add migration script here
ALTER TABLE quests ADD is_active BOOLEAN; 
ALTER TABLE quests ADD is_secondary BOOLEAN;
ALTER TABLE quests ADD is_first_init BOOLEAN;

CREATE TABLE IF NOT EXISTS quest_modifiers (
    id INTEGER PRIMARY KEY,
    quest_id TEXT,
    map_id INTEGER
);