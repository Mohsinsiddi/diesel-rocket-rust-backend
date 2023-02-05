use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{ UserInputCollection}};

// import services module
use crate::services;

#[get("/collections")]
pub fn get_collections() -> Value {
    services::collections::get_collections()
}

#[post("/collections/create-collection", format = "json", data = "<collection_info>")]
pub fn create_collection(collection_info: Json<UserInputCollection>) -> Value {
    services::collections::create_collection(&collection_info)
}
