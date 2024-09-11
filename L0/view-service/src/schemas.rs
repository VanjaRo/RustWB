use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use chrono::{DateTime, Utc};

// In bigger projects it's much more easier to use some ORM solution
// that manages field serialization and deserialization for sql queries

// Chose bigints(i64) for numerical values as I don't really know what range is the most suitable

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Order {
    pub order_uid: String,
    pub track_number: String,
    pub entry: String,
    pub delivery: Delivery,
    pub payment: Payment,
    pub items: Vec<Item>,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i64,
    pub date_created: DateTime<Utc>,
    pub oof_shard: String,
}

impl Order {
    pub fn from_row(row: &Row, delivery: Delivery, payment: Payment, items: Vec<Item>) -> Order {
        Order {
            order_uid: row.get("order_uid"),
            track_number: row.get("track_number"),
            entry: row.get("entry"),
            delivery,
            payment,
            items,
            locale: row.get("locale"),
            internal_signature: row.get("internal_signature"),
            customer_id: row.get("customer_id"),
            delivery_service: row.get("delivery_service"),
            shardkey: row.get("shardkey"),
            sm_id: row.get("sm_id"),
            date_created: row.get("date_created"),
            oof_shard: row.get("oof_shard"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Delivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}

impl Delivery {
    pub fn from_row(row: &Row) -> Delivery {
        Delivery {
            name: row.get("name"),
            phone: row.get("phone"),
            zip: row.get("zip"),
            city: row.get("city"),
            address: row.get("address"),
            region: row.get("region"),
            email: row.get("email"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Payment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: i64,
    pub payment_dt: i64,
    pub bank: String,
    pub delivery_cost: i64,
    pub goods_total: i64,
    pub custom_fee: i64,
}

impl Payment {
    pub fn from_row(row: &Row) -> Payment {
        Payment {
            transaction: row.get("transaction_id"),
            request_id: row.get("request_id"),
            currency: row.get("currency"),
            provider: row.get("provider"),
            amount: row.get("amount"),
            payment_dt: row.get("payment_dt"),
            bank: row.get("bank"),
            delivery_cost: row.get("delivery_cost"),
            goods_total: row.get("goods_total"),
            custom_fee: row.get("custom_fee"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Item {
    pub chrt_id: i64,
    pub track_number: String,
    pub price: i64,
    pub rid: String,
    pub name: String,
    pub sale: i64,
    pub size: String,
    pub total_price: i64,
    pub nm_id: i64,
    pub brand: String,
    pub status: i64,
}

impl Item {
    pub fn from_row(row: &Row) -> Item {
        Item {
            chrt_id: row.get("chrt_id"),
            track_number: row.get("track_number"),
            price: row.get("price"),
            rid: row.get("rid"),
            name: row.get("name"),
            sale: row.get("sale"),
            size: row.get("size"),
            total_price: row.get("total_price"),
            nm_id: row.get("nm_id"),
            brand: row.get("brand"),
            status: row.get("status"),
        }
    }
}
