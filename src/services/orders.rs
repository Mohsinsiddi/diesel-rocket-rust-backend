use diesel::prelude::*;
use workfall_rocket_rs::{
    models::models::{Order, NewOrder, UserInputOrder,Trade, Collection},
    *,
};
use rocket::serde::json::{json, Value};
extern crate bcrypt;

pub fn get_order(order_id :&str) -> Value {
    use workfall_rocket_rs::schema::orders::{dsl::*,id as fetch_order_id};

    let connection = &mut establish_connection();

    let appropriate_filter = order_id.to_string();

    let order: Vec<Order> = orders
    .filter(id.eq(&appropriate_filter))
    .limit(1)
    .load::<Order>(connection)
    .expect("Error loading order");

    let search_order_id = &order[0].id;
    
    let result: Order = orders.filter(fetch_order_id.eq(search_order_id)).get_result::<Order>(connection).unwrap();

    json!(result)
}

pub fn get_orders() -> Value {
    use workfall_rocket_rs::schema::orders::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<Order> = orders.load::<Order>(connection).expect("Error loading posts");

    json!(results)
}

pub fn create_order(order_details: &UserInputOrder) -> Value {
    use workfall_rocket_rs::schema::trades::{dsl::*,id as trade_id};
    use workfall_rocket_rs::schema::collections::{dsl::*,id as collection_id};
    use workfall_rocket_rs::schema::orders;

    let connection = &mut establish_connection();
    
    let appropriate_filter_trade = &order_details.trade_id;
    let appropriate_filter_collection = &order_details.collection_id;


    let mut trade: Vec<Trade> = trades
    .filter(trade_id.eq(&appropriate_filter_trade))
    .limit(1)
    .load::<Trade>(connection)
    .expect("Error loading role");

    let mut collection: Vec<Collection> = collections
    .filter(collection_id.eq(&appropriate_filter_collection))
    .limit(1)
    .load::<Collection>(connection)
    .expect("Error loading role");

    let orderid = uuid::Uuid::new_v4().to_string();

    let new_order : NewOrder = NewOrder {
        id: &orderid,
        trade_id:&mut trade[0].id,
        collection_id:&mut  collection[0].id,
        trade_amount:&order_details.trade_amount,
        rarity:&order_details.rarity,
        collection_root:&order_details.collection_root
    };

    let created_order: Order = diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result::<Order>(connection)
        .expect("Error saving new collection");

    json!(created_order)
}

