// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::DateTime;
use chrono::offset::Utc;
#[derive(Queryable, Debug)]
pub struct Collection {
    pub id: String,
    pub collection_name: String,
    pub ceiling_price: i32,
    pub active_trades: i32,
    pub total_trades: i32,
    pub volume: i32,
    pub supply: i32,
}

#[derive(Queryable, Debug)]
pub struct Order {
    pub id: String,
    pub trade_id: String,
    pub collection_id: String,
    pub trade_amount: i32,
    pub rarity: String,
    pub collection_root: String,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub id: String,
    pub role_name: String,
}

#[derive(Queryable, Debug)]
pub struct Trade {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_by: String,
    pub accepted_order_id: i32,
    pub deposited_amount: i32,
    pub buyer_address: String,
    pub seller_address: String,
    pub created_on: Option<DateTime<Utc>>,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub role_id: String,
    pub password: String,
    pub address: String,
    pub user_name: String,
}

