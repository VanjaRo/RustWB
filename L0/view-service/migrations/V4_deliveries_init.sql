CREATE TABLE IF NOT EXISTS deliveries
(
    order_uid   VARCHAR NOT NULL PRIMARY KEY,
    name        VARCHAR,
    phone       VARCHAR,
    zip         VARCHAR,
    city        VARCHAR,
    address     VARCHAR,
    region      VARCHAR,
    email       VARCHAR,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
        ON DELETE CASCADE
);