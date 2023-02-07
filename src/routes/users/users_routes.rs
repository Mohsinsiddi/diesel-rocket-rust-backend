use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{UserInputRole, UserInputUser, UserInputUpdateUser}};

// import services module
use crate::services;

#[get("/users")]
pub fn get_users() -> Value {
    services::users::users_services::get_users()
}

#[get("/user/<id>")]
pub fn get_user(id: &str) -> Value {
    services::users::users_services::get_user(id)
}

#[get("/user/address/<addr>")]
pub fn get_user_by_address(addr:&str) -> Value {
    services::users::users_services::get_user_by_address(addr)
}

#[get("/user/trades/<id>")]
pub fn get_user_trades(id:&str) -> Value {
    services::users::users_services::get_user_trades(id)
}

#[get("/user/orders/<id>")]
pub fn get_user_orders(id: &str) -> Value {
    services::users::users_services::get_user_orders(id)
}

#[post("/users/add-role", format = "json", data = "<role_info>")]
pub fn create_role(role_info: Json<UserInputRole>) -> Value {
    services::users::users_services::add_role(&role_info.role_name)
}

#[post("/users/create-user", format = "json", data = "<user_info>")]
pub fn create_user(user_info: Json<UserInputUser>) -> Value {
    services::users::users_services::create_user(&user_info)
}

#[put("/users/update", format = "json", data = "<user_info>")]
pub fn update_user(user_info: Json<UserInputUpdateUser>) -> Value {
    services::users::users_services::update_user(&user_info)
}








