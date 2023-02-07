use diesel::prelude::*;
use workfall_rocket_rs::{
    models::models::{NewTrade,Trade,UserInputTrade, User,UserInputUpdateTrade,Order},
    *,
};
use rocket::serde::json::{json, Value};
extern crate bcrypt;

/*
* Get trade details
*/

pub fn get_trade(trade_id :&str) -> Value {
    use workfall_rocket_rs::db::schema::trades::{dsl::*,id as fetch_trade_id};

    let connection = &mut establish_connection();

    let appropriate_filter = trade_id.to_string();

    let trade: Vec<Trade> = trades
    .filter(id.eq(&appropriate_filter))
    .limit(1)
    .load::<Trade>(connection)
    .expect("Error loading trade");

    let search_trade_id = &trade[0].id;

    let result: Trade = trades.filter(fetch_trade_id.eq(search_trade_id)).get_result::<Trade>(connection).unwrap();

    json!(result)
}

/*
* Get All trades details
*/

pub fn get_trades() -> Value {
    use workfall_rocket_rs::db::schema::trades::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<Trade> = trades.load::<Trade>(connection).expect("Error loading posts");

    json!(results)
}

/*
* Get all trade orders details
*/

pub fn get_trade_orders(tra_id:&str) -> Value {
    use workfall_rocket_rs::db::schema::orders::{dsl::*,trade_id as filter_trade_id};

    let connection = &mut establish_connection();

    let result: Vec<Order> = orders.filter(filter_trade_id.eq(tra_id)).get_results::<Order>(connection).unwrap();

    json!(result)
}

/*
* Create trade details
*/

pub fn create_trade(trade_details: &UserInputTrade) -> Value {
    use workfall_rocket_rs::db::schema::trades;
    use workfall_rocket_rs::db::schema::users::{dsl::*,id as user_id};

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
        total_orders:&trade_details.total_orders,
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

/*
* Accept trade details
*/
pub fn accept_trade(trade_details: &UserInputUpdateTrade) -> Value {
    use workfall_rocket_rs::db::schema::trades::{dsl::*,id as trade_id};

    let connection = &mut establish_connection();

    let existing_trade = trades
    .filter(trade_id.eq(trade_details.id.clone()))
    .limit(1)
    .load::<Trade>(connection)
    .expect("Error fetching trade");

    let updated_trade_body: NewTrade = NewTrade {
        id: &existing_trade[0].id,
        total_orders:&trade_details.total_orders.clone().unwrap_or(existing_trade[0].total_orders.clone()),
        created_by: &trade_details.created_by.clone().unwrap_or(existing_trade[0].created_by.clone()),
        accepted_order_id:&trade_details.accepted_order_id.clone().unwrap_or(existing_trade[0].accepted_order_id.clone()),
        deposited_amount:&trade_details.deposited_amount.clone().unwrap_or(existing_trade[0].deposited_amount.clone()),
        buyer_address:&trade_details.buyer_address.clone().unwrap_or(existing_trade[0].buyer_address.clone()),
        seller_address:&trade_details.seller_address.clone().unwrap_or(existing_trade[0].seller_address.clone())
    };
    
    let updated_trade: Trade = diesel::update(trades.filter(trade_id.eq(trade_details.id.clone())))
    .set(&updated_trade_body)
    .get_result::<Trade>(connection)
    .expect("Error accepting trade ");

    json!(updated_trade)
}
