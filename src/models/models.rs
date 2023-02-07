use diesel::{prelude::*};
use rocket::serde::{Serialize, Deserialize};

use crate::db::schema::{roles, users, collections, trades,orders};

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
    pub collection_id_name: String,
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
    pub collection_id_name:&'a str,
    pub ceiling_price: &'a i32 ,
    pub active_trades: &'a i32,
    pub total_trades: &'a i32,
    pub volume: &'a i32,
    pub supply: &'a i32,
}

#[derive(Deserialize)]
pub struct UserInputCollection {
    pub collection_name: String,
    pub collection_id_name: String,
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
    pub total_orders: i32,
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
    pub total_orders:&'a i32,
    pub created_by: &'a str,
    pub accepted_order_id: &'a i32,
    pub deposited_amount: &'a i32,
    pub buyer_address: &'a str,
    pub seller_address: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputTrade {
    pub total_orders: i32,
    pub created_by: String,
    pub accepted_order_id: i32,
    pub deposited_amount: i32,
    pub buyer_address: String,
    pub seller_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInputUpdateTrade {
    pub id : String,
    pub total_orders: Option<i32>,
    pub created_by:  Option<String>,
    pub accepted_order_id:  Option<i32>,
    pub deposited_amount:  Option<i32>,
    pub buyer_address:  Option<String>,
    pub seller_address:  Option<String>,
}

/*
* Order Models begins here
*/

#[derive(Queryable, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub user_id : String,
    pub trade_id: String,
    pub collection_id: String,
    pub trade_amount: i32,
    pub rarity: String,
    pub collection_root: String,
}

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub id:&'a str,
    pub user_id : &'a str,
    pub trade_id: &'a str,
    pub collection_id: &'a str,
    pub trade_amount: &'a i32,
    pub rarity: &'a str,
    pub collection_root: &'a str,
} 

#[derive(Deserialize)]
pub struct UserInputOrder {
    pub user_id: String,
    pub trade_id: String,
    pub collection_id: String,
    pub trade_amount: i32,
    pub rarity: String,
    pub collection_root: String,
}
