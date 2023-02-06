-- Your SQL goes here

CREATE TABLE IF NOT EXISTS collections
(
    id VARCHAR PRIMARY KEY NOT NULL,
    collection_name VARCHAR(30) NOT NULL,
    ceiling_price INT NOT NULL,
    active_trades INT NOT NULL,
    total_trades INT NOT NULL,
    volume INT NOT NULL,
    supply INT NOT NULL
);