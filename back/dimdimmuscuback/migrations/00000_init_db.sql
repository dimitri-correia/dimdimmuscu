-- sqlite

PRAGMA foreign_keys = ON;

CREATE TABLE users
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    name             VARCHAR(256) NOT NULL,
    birthdate        TEXT         NOT NULL,
    account_creation TEXT         NOT NULL,
    CONSTRAINT unique_name UNIQUE (name)
);

-- Create an index to have faster queries on name
CREATE INDEX idx_users_name ON users (name);

CREATE TABLE users_auth
(
    profile_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE PRIMARY KEY,
    pwd        VARCHAR(256),
    CONSTRAINT unique_profile_id UNIQUE (profile_id)
);

CREATE TABLE session
(
    token      VARCHAR(256) NOT NULL PRIMARY KEY,
    profile_id INTEGER      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    until      TEXT         NOT NULL
);

CREATE TABLE muscles
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(256) NOT NULL,
    CONSTRAINT unique_muscle_name UNIQUE (name)
);

CREATE TABLE movements
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          VARCHAR(256) NOT NULL,
    first_muscle  INTEGER      NOT NULL REFERENCES muscles (id),
    second_muscle INTEGER REFERENCES muscles (id),
    third_muscle  INTEGER REFERENCES muscles (id),
    CONSTRAINT unique_movement_name UNIQUE (name)
);

CREATE TABLE lifts
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    lift_time   TEXT
);

CREATE TABLE sets
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    lift_id    INTEGER NOT NULL REFERENCES lifts (id) ON DELETE CASCADE,
    set_number INTEGER NOT NULL,
    number_rep INTEGER NOT NULL,
    weight     INTEGER NOT NULL,
    CONSTRAINT unique_set_number_per_lift UNIQUE (set_number, lift_id)
);

CREATE TABLE maxes
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    number_rep  INTEGER NOT NULL,
    weight      INTEGER NOT NULL,
    CONSTRAINT unique_max_per_profile_movement UNIQUE (profile_id, movement_id)
);
