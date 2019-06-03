CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT 0
);
