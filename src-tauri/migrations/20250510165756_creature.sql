-- Add migration script here
CREATE TABLE IF NOT EXISTS creatures (
    id INT PRIMARY KEY,
    attack INT,
    defence INT,
    min_damage INT,
    max_damage INT,
    speed INT,
    initiative INT,
    is_flying BOOLEAN,
    known_spells TEXT,
    spell_points INT,
    exp INT,
    power INT,
    tier INT,
    town TEXT,
    magic_element TEXT,
    grow INT,
    cost TEXT,
    is_generatable BOOLEAN,
    shared TEXT,
    size INT,
    range INT,
    name_txt TEXT,
    name TEXT,
    desc_txt TEXT,
    desc TEXT,
    icon_xdb TEXT,
    abilities TEXT
);