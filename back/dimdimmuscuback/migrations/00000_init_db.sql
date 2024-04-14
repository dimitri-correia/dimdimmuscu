-- sqlite

PRAGMA foreign_keys = ON;

CREATE TABLE users
(
    id               TEXT PRIMARY KEY,
    name             TEXT NOT NULL,
    birthdate        TEXT NOT NULL,
    account_creation TEXT NOT NULL,
    CONSTRAINT unique_name UNIQUE (name)
);

-- Create an index to have faster queries on name
CREATE INDEX idx_users_name ON users (name);

CREATE TABLE users_auth
(
    profile_id TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE PRIMARY KEY,
    pwd        TEXT,
    CONSTRAINT unique_profile_id UNIQUE (profile_id)
);

CREATE TABLE session
(
    token      TEXT NOT NULL PRIMARY KEY,
    profile_id TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    until      TEXT NOT NULL
);

CREATE TABLE muscles
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    CONSTRAINT unique_muscle_name UNIQUE (name)
);

CREATE TABLE movements
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT    NOT NULL,
    first_muscle  INTEGER NOT NULL REFERENCES muscles (id),
    second_muscle INTEGER REFERENCES muscles (id),
    third_muscle  INTEGER REFERENCES muscles (id),
    CONSTRAINT unique_movement_name UNIQUE (name)
);

CREATE TABLE lifts
(
    id          TEXT PRIMARY KEY,
    profile_id  TEXT    NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    lift_time   TEXT
);

CREATE TABLE sets
(
    id         TEXT PRIMARY KEY,
    lift_id    TEXT    NOT NULL REFERENCES lifts (id) ON DELETE CASCADE,
    set_number INTEGER NOT NULL,
    number_rep INTEGER NOT NULL,
    weight     INTEGER NOT NULL,
    CONSTRAINT unique_set_number_per_lift UNIQUE (set_number, lift_id)
);

CREATE TABLE maxes
(
    id          TEXT PRIMARY KEY,
    profile_id  TEXT    NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    number_rep  INTEGER NOT NULL,
    weight      INTEGER NOT NULL,
    CONSTRAINT unique_max_per_profile_movement UNIQUE (profile_id, movement_id)
);
