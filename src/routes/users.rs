use rocket::serde::json::{Value, Json};
use workfall_rocket_rs::{models::models::{UserInputRole, UserInputUser, UserInputUpdateUser}};

// import services module
use crate::services;

#[get("/users")]
pub fn get_users() -> Value {
    services::users::get_users()
}

#[get("/user/<id>")]
pub fn get_user(id: &str) -> Value {
    services::users::get_user(id)
}

#[get("/user/trades/<id>")]
pub fn get_user_trades(id:&str) -> Value {
    services::users::get_user_trades(id)
}

#[get("/user/orders/<id>")]
pub fn get_user_orders(id: &str) -> Value {
    services::users::get_user_orders(id)
}

#[post("/users/add-role", format = "json", data = "<role_info>")]
pub fn create_role(role_info: Json<UserInputRole>) -> Value {
    services::users::add_role(&role_info.role_name)
}

#[post("/users/create-user", format = "json", data = "<user_info>")]
pub fn create_user(user_info: Json<UserInputUser>) -> Value {
    services::users::create_user(&user_info)
}

#[put("/users/update", format = "json", data = "<user_info>")]
pub fn update_user(user_info: Json<UserInputUpdateUser>) -> Value {
    services::users::update_user(&user_info)
}








