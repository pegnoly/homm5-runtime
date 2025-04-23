-- Add migration script here
CREATE TABLE IF NOT EXISTS banks (
    id INTEGER PRIMARY KEY,
    type INTEGER,
    name TEXT,
    recharge_count INTEGER DEFAULT -1,
    recharge_timer INTEGER DEFAULT 14,
    luck_loss INTEGER DEFAULT 0,
    morale_loss INTEGER DEFAULT 0
);

INSERT INTO banks(type, name) VALUES(0, "Crypt");
INSERT INTO banks(type, name) VALUES(1, "Pyramid");
INSERT INTO banks(type, name) VALUES(2, "Magi vault");
INSERT INTO banks(type, name) VALUES(3, "Dragon utopia");
INSERT INTO banks(type, name) VALUES(4, "Elemental stockpile");
INSERT INTO banks(type, name) VALUES(5, "Dwarven treasure");
INSERT INTO banks(type, name) VALUES(6, "Blood temple");
INSERT INTO banks(type, name) VALUES(7, "Treant thicket");
INSERT INTO banks(type, name) VALUES(8, "Gargoyles stonevault");
INSERT INTO banks(type, name) VALUES(9, "Sunken temple");

CREATE TABLE IF NOT EXISTS bank_variants(
    id INTEGER PRIMARY KEY,
    bank_id INTEGER,
    chance INTEGER,
    difficulty INTEGER,
    FOREIGN KEY (bank_id) REFERENCES banks(id)
);