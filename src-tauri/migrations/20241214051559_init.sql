-- Add migration script here
CREATE TABLE IF NOT EXISTS quests (
    id INTEGER UNIQUE PRIMARY KEY,
    directory TEXT,
    campaign_number INTEGER,
    mission_number INTEGER,
    name TEXT,
    desc TEXT
);

CREATE TABLE IF NOT EXISTS progresses (
    id INTEGER UNIQUE PRIMARY KEY,
    quest_id INTEGER,
    number INTEGER,
    text TEXT,
    FOREIGN KEY(quest_id) REFERENCES quests(id)
);