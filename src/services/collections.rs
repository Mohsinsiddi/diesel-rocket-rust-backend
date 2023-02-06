use diesel::prelude::*;
use workfall_rocket_rs::{
    models::models::{NewCollection,Collection,UserInputCollection,Order},
    *,
};
use rocket::serde::json::{json, Value};
extern crate bcrypt;

/*
* Get collection details
*/

pub fn get_collection(collection_id :&str) -> Value {
    use workfall_rocket_rs::schema::collections::{dsl::*,id as fetch_collection_id};

    let connection = &mut establish_connection();

    let appropriate_filter = collection_id.to_string();

    let collection: Vec<Collection> = collections
    .filter(id.eq(&appropriate_filter))
    .limit(1)
    .load::<Collection>(connection)
    .expect("Error loading user");
    let search_collection_id = &collection[0].id;

    let result: Collection = collections.filter(fetch_collection_id.eq(search_collection_id)).get_result::<Collection>(connection).unwrap();

    json!(result)
}

/*
* Get all collection orders details
*/

pub fn get_collection_orders(col_id:&str) -> Value {
    
    use workfall_rocket_rs::schema::orders::{dsl::*,collection_id as filter_collection_id};

    let connection = &mut establish_connection();

    let result: Vec<Order> = orders.filter(filter_collection_id.eq(col_id)).get_results::<Order>(connection).unwrap();

    json!(result)
}

/*
* Get all collection details
*/

pub fn get_collections() -> Value {
    use workfall_rocket_rs::schema::collections::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<Collection> = collections.load::<Collection>(connection).expect("Error loading posts");

    json!(results)
}

/*
* Create collection details
*/

pub fn create_collection(collection_details: &UserInputCollection) -> Value {
    use workfall_rocket_rs::schema::collections;

    let connection = &mut establish_connection();

    let collectionid = uuid::Uuid::new_v4().to_string();

    let new_collection : NewCollection = NewCollection {
        id : &collectionid,
        collection_name:&collection_details.collection_name,
        ceiling_price:&collection_details.ceiling_price,
        active_trades:&collection_details.active_trades,
        total_trades:&collection_details.total_trades,
        volume:&collection_details.volume,
        supply:&collection_details.supply
    };

    let created_collection: Collection = diesel::insert_into(collections::table)
        .values(&new_collection)
        .get_result::<Collection>(connection)
        .expect("Error saving new collection");

    json!(created_collection)
}

