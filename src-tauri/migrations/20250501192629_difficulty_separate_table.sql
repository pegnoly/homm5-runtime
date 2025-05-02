-- Add migration script here
ALTER TABLE bank_variants DROP chance;
ALTER TABLE bank_variants ADD label TEXT;

CREATE TABLE IF NOT EXISTS bank_difficulties(
    id INT PRIMARY KEY,
    bank_id INT,
    difficulty_type INT,
    chance INT
);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (0, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (0, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (0, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (0, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (0, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (1, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (1, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (1, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (1, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (1, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (2, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (2, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (2, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (2, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (2, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (3, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (3, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (3, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (3, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (3, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (4, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (4, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (4, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (4, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (4, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (5, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (5, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (5, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (5, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (5, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (6, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (6, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (6, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (6, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (6, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (7, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (7, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (7, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (7, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (7, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (8, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (8, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (8, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (8, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (8, 4, 20);

INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (9, 0, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (9, 1, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (9, 2, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (9, 3, 20);
INSERT INTO bank_difficulties(bank_id, difficulty_type, chance) VALUES (9, 4, 20);