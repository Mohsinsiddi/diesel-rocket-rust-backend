use diesel::prelude::*;
use workfall_rocket_rs::{
    models::models::{NewTrade,Trade,UserInputTrade, User},
    *,
};
use rocket::serde::json::{json, Value};
extern crate bcrypt;

pub fn get_trades() -> Value {
    use workfall_rocket_rs::schema::trades::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<Trade> = trades.load::<Trade>(connection).expect("Error loading posts");

    json!(results)
}

pub fn create_trade(trade_details: &UserInputTrade) -> Value {
    use workfall_rocket_rs::schema::trades;
    use workfall_rocket_rs::schema::users::{dsl::*,id as user_id};

    let connection = &mut establish_connection();
    
    let appropriate_filter = &trade_details.created_by;


    let mut user: Vec<User> = users
    .filter(user_id.eq(&appropriate_filter))
    .limit(1)
    .load::<User>(connection)
    .expect("Error loading role");

    let tradeid = uuid::Uuid::new_v4().to_string();

    let new_trade : NewTrade = NewTrade {
        id: &tradeid,
        title:&trade_details.title,
        content:&trade_details.content,
        created_by: &mut user[0].id,
        accepted_order_id:&trade_details.accepted_order_id,
        deposited_amount:&trade_details.deposited_amount,
        buyer_address:&trade_details.buyer_address,
        seller_address:&trade_details.seller_address
    };

    let created_trade: Trade = diesel::insert_into(trades::table)
        .values(&new_trade)
        .get_result::<Trade>(connection)
        .expect("Error saving new collection");

    json!(created_trade)
}

