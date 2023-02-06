#[macro_use] extern crate rocket;
use rocket::{serde::{json::{json, Value}}};

// add routes and services modules here
mod routes;
mod services;

// import routes here
use routes::index::index;
use routes::users::{get_users, create_role, create_user, update_user,get_user};
use routes::collections::{get_collections,create_collection,get_collection};
use routes::trades::{get_trades,create_trade,get_trade};
use routes::orders::{get_orders,create_order,get_order};

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
fn server_error() -> Value {
    json!({
        "status": "Server error",
        "reason": "Something went wrong. Please try again later"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, 
              get_users,
              get_user,
              create_role,
              create_user, 
              update_user,
              get_collections,
              get_collection,
              create_collection,
              get_trades,
              get_trade,
              create_trade,
              get_orders,
              get_order,
              create_order])
    .register("/", catchers![not_found, server_error])
}
