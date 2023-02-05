-- Your SQL goes here

CREATE TABLE trades (
  id VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  content text NOT NULL,
  created_by VARCHAR NOT NULL,
  accepted_order_id INT NOT NULL,
  deposited_amount INT NOT NULL,
  buyer_address VARCHAR NOT NULL,
  seller_address VARCHAR NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (created_by) REFERENCES users(id)
);
