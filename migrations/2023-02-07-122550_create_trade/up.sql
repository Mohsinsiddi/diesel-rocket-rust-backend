-- Your SQL goes here

CREATE TABLE trades (
  id VARCHAR NOT NULL,
  total_orders INT NOT NULL, -- this is added
  created_by VARCHAR NOT NULL,
  accepted_order_id INT NOT NULL,
  deposited_amount INT NOT NULL,
  buyer_address VARCHAR NOT NULL,
  seller_address VARCHAR NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (created_by) REFERENCES users(id)
);
