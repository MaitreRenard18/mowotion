CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE session (
    id INTEGER NOT NULL,
    session_token UNIQUE TEXT NOT NULL,
    csrf_token TEXT UNIQUE NOT NULL,

    PRIMARY KEY (id, session_token, csrf_token),
    FOREIGN KEY (id) REFERENCES user (id)
);