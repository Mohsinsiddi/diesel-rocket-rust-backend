use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{UserInputTrade,UserInputUpdateTrade}};

// import services module
use crate::services;

#[get("/trades")]
pub fn get_trades() -> Value {
    services::trades::trades_services::get_trades()
}

#[get("/trade/<id>")]
pub fn get_trade(id:&str) -> Value {
    services::trades::trades_services::get_trade(id)
}

#[get("/trade/orders/<id>")]
pub fn get_trade_orders(id:&str) -> Value {
    services::trades::trades_services::get_trade_orders(id)
}

#[post("/trades/create-trade", format = "json", data = "<trade_info>")]
pub fn create_trade(trade_info: Json<UserInputTrade>) -> Value {
    services::trades::trades_services::create_trade(&trade_info)
}

#[post("/trade/accept_trade", format = "json", data = "<trade_info>")]
pub fn accept_trade(trade_info: Json<UserInputUpdateTrade>) -> Value {
    services::trades::trades_services::accept_trade(&trade_info)
}


