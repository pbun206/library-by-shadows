CREATE TABLE IF NOT EXISTS users
(
    uuid TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    is_account_enabled BOOLEAN NOT NULL DEFAULT 1,
    web_theme TEXT NOT NULL DEFAULT 'system',
    is_admin BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS listed_rules
(
    uuid TEXT PRIMARY KEY,
    owner_uuid TEXT NOT NULL,
    img_url TEXT NULL,
    rule_contents TEXT NULL
);

CREATE TABLE IF NOT EXISTS url
(
    url TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    content TEXT NOT NULL,
    first_indexed_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    last_indexed_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    last_published_at INTEGER NULL,
    last_edited_at INTEGER NULL
);
