use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{UserInputOrder}};

// import services module
use crate::services;

#[get("/orders")]
pub fn get_orders() -> Value {
    services::orders::get_orders()
}

#[post("/orders/create-order", format = "json", data = "<order_info>")]
pub fn create_order(order_info: Json<UserInputOrder>) -> Value {
    services::orders::create_order(&order_info)
}