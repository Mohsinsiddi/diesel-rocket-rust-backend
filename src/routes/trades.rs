use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{UserInputTrade}};

// import services module
use crate::services;

#[get("/trades")]
pub fn get_trades() -> Value {
    services::trades::get_trades()
}

#[post("/trades/create-trades", format = "json", data = "<trade_info>")]
pub fn create_trade(trade_info: Json<UserInputTrade>) -> Value {
    services::trades::create_trade(&trade_info)
}
