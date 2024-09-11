CREATE TABLE IF NOT EXISTS orders
(
   order_uid            VARCHAR NOT NULL PRIMARY KEY,
   track_number         VARCHAR,
   entry                VARCHAR,
   locale               VARCHAR,
   internal_signature   VARCHAR,
   customer_id          VARCHAR,
   delivery_service     VARCHAR,
   shardkey             VARCHAR,
   sm_id                BIGINT,
   date_created         TIMESTAMP WITH TIME ZONE,
   oof_shard            VARCHAR
);