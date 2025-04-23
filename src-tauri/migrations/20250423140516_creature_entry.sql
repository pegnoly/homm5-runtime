-- Add migration script here
CREATE TABLE IF NOT EXISTS bank_creature_entries (
    id INTEGER PRIMARY KEY,
    variant_id INTEGER,
    type INTEGER,
    data TEXT,
    FOREIGN KEY (variant_id) REFERENCES bank_variants(id)
);