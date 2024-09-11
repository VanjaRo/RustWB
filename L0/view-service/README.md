# View service

## Description

Simple service that serves as an entry point for the WBTech Rust course. You send a json file with a model described in this [file](./model/model.json) and later can view it.  

Routes:

- GET to `order/:order_uid` returns an order if exists.
- POST to `order` with JSON body creates an order.

## Development

### Startup

To start a db use `docker compose up`.  
To start the service run `cargo build --release` and `./target/release/view-service`  
You can configure:

- Server Port, default: 3000,
- Postgres DB Host, default: localhost,
- Postgres DB Port, default: 5432

`POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB` values are by default set to `postgres`, you can set the by yourself throug env variables.

### Considerations

- The task states that the orders are immutable so there are reasons to store it as a single JSON per order, however analitical demands for the platform are not clear and bringing filtering for the service might be hard with JSON storing style.
- While receiving a JSON all extra fields that are not included in the schema are ignored by the service.
