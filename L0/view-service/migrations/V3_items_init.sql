CREATE TABLE IF NOT EXISTS items
(
    order_uid       VARCHAR NOT NULL,
    chrt_id         BIGINT,
    track_number    VARCHAR,
    price           BIGINT,
    rid             VARCHAR,
    name            VARCHAR,
    sale            BIGINT,
    size            VARCHAR,
    total_price     BIGINT,
    nm_id           BIGINT,
    brand           VARCHAR,
    status          BIGINT,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
        ON DELETE CASCADE
);
