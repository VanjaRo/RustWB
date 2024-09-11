use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};

use itertools::Itertools;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use clap::Parser;
use schemas::{Delivery, Item, Order, Payment};
use tokio_postgres::{types::ToSql, Config, NoTls};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod schemas;

type DbPoolNoTsl = Pool<PostgresConnectionManager<NoTls>>;
// currrently using simple HashMap, it might be a good idea to use LRU
type RwCacheOrder = Arc<RwLock<HashMap<String, schemas::Order>>>;

#[derive(Parser, Debug)]
struct Args {
    // Port to start a server at
    #[clap(short = 'p', long, default_value = "3000")]
    server_port: String,

    // Port to listen to a PostgreSQL DB
    #[clap(short = 'd', long, default_value = "5432")]
    pg_port: u16,

    // Host to listen to a PostgreSQL DB
    #[clap(short = 'l', long, default_value = "localhost")]
    pg_host: String,
}

const DEFAULT_POSTGRES_USER: &str = "postgres";
// it looks like a perfect vulnerability to hack into DB
const DEFAULT_POSTGRES_PASSWORD: &str = "postgres";
const DEFAULT_POSTGRES_DB: &str = "postgres";

#[derive(Clone)]
struct AppState {
    pool: DbPoolNoTsl,
    cache: RwCacheOrder,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // init DB connection
    let mut db_config = Config::new();
    db_config.user(match env::var("POSTGRES_USER") {
        Ok(usr) => usr,
        Err(_) => DEFAULT_POSTGRES_USER.to_string(),
    });
    db_config.password(match env::var("POSTGRES_PASSWORD") {
        Ok(pswd) => pswd,
        Err(_) => DEFAULT_POSTGRES_PASSWORD.to_string(),
    });
    db_config.dbname(match env::var("POSTGRES_DB") {
        Ok(dbname) => dbname,
        Err(_) => DEFAULT_POSTGRES_DB.to_string(),
    });
    db_config.port(args.pg_port);
    db_config.host(args.pg_host);

    tracing::debug!("built a db config with values {:?}", db_config);

    let manager = PostgresConnectionManager::new(db_config, NoTls);
    let pool: Pool<PostgresConnectionManager<NoTls>> =
        Pool::builder().build(manager).await.unwrap();

    // init cache layer
    let cache = Arc::new(RwLock::new(HashMap::new()));
    // create new state
    let app_state = Arc::new(AppState { pool, cache });

    // start server
    let app = Router::new()
        .route("/order/:order_uid", get(get_order))
        .route("/order", post(create_order))
        .with_state(app_state);

    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{port}", port = args.server_port))
            .await
            .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// process order post
async fn create_order(
    State(state): State<Arc<AppState>>,
    axum::Json(order): axum::Json<schemas::Order>,
) -> Response {
    tracing::debug!("order post request");
    match inser_order_tx(&order, state.clone()).await {
        Ok(_) => {
            let order_uid = order.order_uid.clone();
            let _ = state.cache.write().unwrap().insert(order_uid, order);
            tracing::debug!("transaction commited");
            (
                StatusCode::CREATED,
                "Order successfully created".to_string(),
            )
                .into_response()
        }
        Err(e) => {
            tracing::debug!("transaction reverted");
            e.into_response()
        }
    }
}

async fn inser_order_tx(order: &Order, state: Arc<AppState>) -> Result<(), (StatusCode, String)> {
    let mut conn = state.pool.get().await.map_err(internal_error).unwrap();

    let transaction = conn.transaction().await.map_err(internal_error)?;
    tracing::debug!("transaction started");

    let order_query =
    "INSERT INTO orders (order_uid, track_number, entry, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
    let _order_result = transaction
        .execute(
            order_query,
            &[
                &order.order_uid,
                &order.track_number,
                &order.entry,
                &order.locale,
                &order.internal_signature,
                &order.customer_id,
                &order.delivery_service,
                &order.shardkey,
                &order.sm_id,
                &order.date_created,
                &order.oof_shard,
            ],
        )
        .await
        .map_err(internal_error)?;

    let delivery_query =
        "INSERT INTO deliveries (order_uid, name, phone, zip, city, address, region, email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    let delivery = &order.delivery;
    let _delivery_result = transaction
        .execute(
            delivery_query,
            &[
                &order.order_uid,
                &delivery.name,
                &delivery.phone,
                &delivery.zip,
                &delivery.city,
                &delivery.address,
                &delivery.region,
                &delivery.email,
            ],
        )
        .await
        .map_err(internal_error)?;

    tracing::debug!("performed delivery insertion");

    let items_params: Vec<_> = order
        .items
        .iter()
        .flat_map(|item| {
            [
                &order.order_uid as &(dyn ToSql + Sync),
                &item.chrt_id,
                &item.track_number,
                &item.price,
                &item.rid,
                &item.name,
                &item.sale,
                &item.size,
                &item.total_price,
                &item.nm_id,
                &item.brand,
                &item.status,
            ]
        })
        .collect();
    // preparing placeholders for the query params
    let items_query = format!(
        "INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status) VALUES {}",
        (1..items_params.len()+1)
            .tuples()
            .format_with(", ", |(a, b, c, d, e, ff, g, h, i, j, k, l), f| {
                f(&format_args!("(${a}, ${b}, ${c},  ${d}, ${e}, ${ff}, ${g}, ${h}, ${i}, ${j}, ${k}, ${l})"))
            }),
    );
    let _items_res = transaction
        .execute(items_query.as_str(), &items_params)
        .await
        .map_err(internal_error)?;

    tracing::debug!("performed items insertion");

    let payment = &order.payment;
    let payment_query =
        "INSERT INTO payments (order_uid, transaction_id, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
    let _payment_result = transaction
        .execute(
            payment_query,
            &[
                &order.order_uid,
                &payment.transaction,
                &payment.request_id,
                &payment.currency,
                &payment.provider,
                &payment.amount,
                &payment.payment_dt,
                &payment.bank,
                &payment.delivery_cost,
                &payment.goods_total,
                &payment.custom_fee,
            ],
        )
        .await
        .map_err(internal_error)?;

    tracing::debug!("performed payment insertion");

    transaction.commit().await.map_err(internal_error)?;
    Ok(())
}

// process order get
async fn get_order(Path(order_uid): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    // check cahce
    {
        tracing::debug!("checking cache for order with uid: {}", order_uid);
        let cache = state.cache.read().map_err(internal_error).unwrap();
        if let Some(order) = cache.get(&order_uid) {
            return (StatusCode::OK, Json(order)).into_response();
        }
    }

    tracing::debug!("no cahce hit");

    // no cache hit
    match collect_order(order_uid, state.clone()).await {
        Ok(order) => {
            let order_uid = order.order_uid.clone();
            let _ = state
                .cache
                .write()
                .unwrap()
                .insert(order_uid, order.clone());
            (StatusCode::OK, Json(order)).into_response()
        }
        Err(e) => e.into_response(),
    }
}

async fn collect_order(
    order_uid: String,
    state: Arc<AppState>,
) -> Result<Order, (StatusCode, String)> {
    let conn = state.pool.get().await.map_err(internal_error).unwrap();
    // request items
    let items_rows = conn
        .query("SELECT * FROM items WHERE order_uid = $1", &[&order_uid])
        .await
        .map_err(not_found_err)?;

    let items = items_rows
        .into_iter()
        .map(|row| Item::from_row(&row))
        .collect::<Vec<_>>();

    // request delivery
    let delivery_row = conn
        .query_one(
            "SELECT * FROM deliveries WHERE order_uid = $1",
            &[&order_uid],
        )
        .await
        .map_err(not_found_err)?;
    let delivery = Delivery::from_row(&delivery_row);

    // request payment
    let payment_row = conn
        .query_one("SELECT * FROM payments WHERE order_uid = $1", &[&order_uid])
        .await
        .map_err(not_found_err)?;
    let payment = Payment::from_row(&payment_row);

    let order_row = conn
        .query_one("SELECT * FROM orders WHERE order_uid = $1", &[&order_uid])
        .await
        .map_err(not_found_err)?;

    Ok(Order::from_row(&order_row, delivery, payment, items))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn not_found_err<E>(_: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (
        StatusCode::NOT_FOUND,
        "The order has not been found in the sistem".to_string(),
    )
}
