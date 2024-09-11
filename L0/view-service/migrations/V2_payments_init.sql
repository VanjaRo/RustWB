-- choosing bifint for the prices as some countries might have
-- wierd inflation rate
CREATE TABLE IF NOT EXISTS payments
(
    order_uid       VARCHAR NOT NULL PRIMARY KEY,
    transaction_id  VARCHAR NOT NULL,
    request_id      VARCHAR,
    currency        VARCHAR,
    provider        VARCHAR,
    amount          BIGINT,
    payment_dt      BIGINT,
    bank            VARCHAR,
    delivery_cost   BIGINT,
    goods_total     BIGINT,
    custom_fee      BIGINT,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
        ON DELETE CASCADE
);