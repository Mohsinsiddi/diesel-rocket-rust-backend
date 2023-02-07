use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{ UserInputCollection}};

// import services module
use crate::services;

#[get("/collections")]
pub fn get_collections() -> Value {
    services::collections::collections_services::get_collections()
}

#[get("/collection/<id>")]
pub fn get_collection(id:&str) -> Value {
    services::collections::collections_services::get_collection(id)
}

#[get("/collection/orders/<id>")]
pub fn get_collection_orders(id:&str) -> Value {
    services::collections::collections_services::get_collection_orders(id)
}

#[post("/collections/create-collection", format = "json", data = "<collection_info>")]
pub fn create_collection(collection_info: Json<UserInputCollection>) -> Value {
    services::collections::collections_services::create_collection(&collection_info)
}
