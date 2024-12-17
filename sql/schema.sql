CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE session (
    session_token TEXT UNIQUE NOT NULL,
    user_id INTEGER PRIMARY KEY NOT NULL,
    expire_date DATE NOT NULL,

    FOREIGN KEY (user_id) REFERENCES user (id)
);