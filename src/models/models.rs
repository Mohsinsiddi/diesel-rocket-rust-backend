use diesel::{prelude::*};
use rocket::serde::{Serialize, Deserialize};

use crate::schema::{roles, users, collections, trades};

/*
* User models begin from here
*/

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub role_id: String,
    pub password: String,
    pub address: String,
    pub user_name: String
}

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub first_name: &'a str,
    pub middle_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub role_id: &'a str,
    pub password: &'a str,
    pub address: &'a str,
    pub user_name: &'a str
}

#[derive(Deserialize)]
pub struct UserInputUser {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: Option<String>,
    pub password: String,
    pub address : String,
    pub user_name: String
}

#[derive(Deserialize, Serialize)]
pub struct UserInputUpdateUser {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role_id: Option<String>,
    pub password: Option<String>,
    pub address : Option<String>,
    pub user_name: Option<String>
}

/*
* Role models (no pun intended) begin from here
*/

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub role_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub id: &'a str,
    pub role_name: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputRole {
    pub role_name: String,
}

/*
* Collection Models begin from here
*/
#[derive(Queryable, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub collection_name: String,
    pub ceiling_price: i32,
    pub active_trades: i32,
    pub total_trades: i32,
    pub volume: i32,
    pub supply: i32,
}

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = collections)]
pub struct NewCollection<'a> {
    pub id: &'a str,
    pub collection_name: &'a str,
    pub ceiling_price: &'a i32 ,
    pub active_trades: &'a i32,
    pub total_trades: &'a i32,
    pub volume: &'a i32,
    pub supply: &'a i32,
}

#[derive(Deserialize)]
pub struct UserInputCollection {
    pub collection_name: String,
    pub ceiling_price: i32,
    pub active_trades: i32,
    pub total_trades: i32,
    pub volume: i32,
    pub supply: i32,
}
/*
* Trade Models begin from here
 */

#[derive(Queryable, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_by: String,
    pub accepted_order_id: i32,
    pub deposited_amount: i32,
    pub buyer_address: String,
    pub seller_address: String,
}

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = trades)]
pub struct NewTrade<'a> {
    pub id:&'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub created_by: &'a str,
    pub accepted_order_id: &'a i32,
    pub deposited_amount: &'a i32,
    pub buyer_address: &'a str,
    pub seller_address: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputTrade {
    pub title: String,
    pub content: String,
    pub created_by: String,
    pub accepted_order_id: i32,
    pub deposited_amount: i32,
    pub buyer_address: String,
    pub seller_address: String,
}
