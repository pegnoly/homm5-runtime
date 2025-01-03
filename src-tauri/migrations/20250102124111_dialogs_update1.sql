-- Add migration script here
DROP TABLE dialog_variants;

CREATE TABLE IF NOT EXISTS variants 
(
    id TEXT PRIMARY KEY,
    dialog_id TEXT,
    step INTEGER,
    label TEXT,
    speaker_id TEXT,
    text TEXT,
    FOREIGN KEY(dialog_id) REFERENCES dialogs(id)
);