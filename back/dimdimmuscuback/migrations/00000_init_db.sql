CREATE TABLE users
(
    id        SERIAL PRIMARY KEY,
    name      VARCHAR NOT NULL,
    birthdate DATE    NOT NULL,
    CONSTRAINT unique_name UNIQUE (name)
);

CREATE TABLE muscles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    CONSTRAINT unique_muscle_name UNIQUE (name)
);

CREATE TABLE movements
(
    id            SERIAL PRIMARY KEY,
    name          VARCHAR NOT NULL,
    first_muscle  INTEGER NOT NULL REFERENCES muscles (id),
    second_muscle INTEGER REFERENCES muscles (id),
    third_muscle  INTEGER REFERENCES muscles (id),
    CONSTRAINT unique_movement_name UNIQUE (name)
);

CREATE TABLE lifts
(
    id          SERIAL PRIMARY KEY,
    profile_id  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    lift_time   TIMESTAMP
);

CREATE TABLE sets
(
    id         SERIAL PRIMARY KEY,
    lift_id    INTEGER NOT NULL REFERENCES lifts (id) ON DELETE CASCADE,
    set_number INTEGER NOT NULL,
    number_rep INTEGER NOT NULL,
    weight     INTEGER NOT NULL,
    CONSTRAINT unique_set_number_per_lift UNIQUE (set_number, lift_id)
);

CREATE TABLE maxes
(
    id          SERIAL PRIMARY KEY,
    profile_id  INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    movement_id INTEGER NOT NULL REFERENCES movements (id),
    number_rep  INTEGER NOT NULL,
    weight      INTEGER NOT NULL,
    CONSTRAINT unique_max_per_profile_movement UNIQUE (profile_id, movement_id)
);
