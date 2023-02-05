-- Your SQL goes here

CREATE TABLE IF NOT EXISTS orders 
(
    id VARCHAR PRIMARY KEY NOT NULL,
    trade_id VARCHAR NOT NULL,
    collection_id VARCHAR NOT NULL,
    trade_amount INT NOT NULL,
    rarity VARCHAR(255) NOT NULL,
    collection_root VARCHAR(255) NOT NULL,
    FOREIGN KEY (trade_id) REFERENCES trades(id) ON DELETE CASCADE,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
)