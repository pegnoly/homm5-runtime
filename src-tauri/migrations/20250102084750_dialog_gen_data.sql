-- Add migration script here
CREATE TABLE IF NOT EXISTS dialogs 
(
    id TEXT PRIMARY KEY,
    name TEXT, 
    script_name TEXT,
    directory TEXT,
    speakers_ids TEXT,
    labels TEXT
);

CREATE TABLE IF NOT EXISTS dialog_variants 
(
    id INTEGER PRIMARY KEY,
    label TEXT,
    speaker_id TEXT,
    text TEXT,
    counter INTEGER,
    dialog_id TEXT,
    FOREIGN KEY(dialog_id) REFERENCES dialogs(id)
);

CREATE TABLE IF NOT EXISTS speakers 
(
    id TEXT PRIMARY KEY,
    name TEXT,
    script_name TEXT,
    speaker_type INTEGER,
    color TEXT
)