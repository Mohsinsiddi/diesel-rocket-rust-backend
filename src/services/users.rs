use diesel::prelude::*;
use workfall_rocket_rs::{
    models::models::{NewRole, NewUser, Role, User, UserInputUser, UserInputUpdateUser,Trade},
    *,
};
use rocket::serde::json::{json, Value};
extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

/*
* Get user details
*/

pub fn get_user(user_id :&str) -> Value {
    use workfall_rocket_rs::schema::users::{dsl::*,id as fetch_user_id};

    let connection = &mut establish_connection();

    let appropriate_filter = user_id.to_string();

    let user: Vec<User> = users
    .filter(id.eq(&appropriate_filter))
    .limit(1)
    .load::<User>(connection)
    .expect("Error loading user");
    let search_user_id = &user[0].id;

    let result: User = users.filter(fetch_user_id.eq(search_user_id)).get_result::<User>(connection).unwrap();

    json!(result)
}

/*
* Get All user trade details
*/

pub fn get_user_trades( user_id:&str) -> Value {
    use workfall_rocket_rs::schema::trades::{dsl::*,id as trade_id,created_by as created_by_id};

    let connection = &mut establish_connection();

    let result: Vec<Trade> = trades.filter(created_by_id.eq(user_id)).get_results::<Trade>(connection).unwrap();

    json!(result)
}

/*
* Get All users details
*/

pub fn get_users() -> Value {
    use workfall_rocket_rs::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<User> = users.load::<User>(connection).expect("Error loading posts");

    json!(results)
}

/*
* Add Role details
*/

pub fn add_role(role_name: &str) -> Value {
    use workfall_rocket_rs::schema::roles;

    let id = uuid::Uuid::new_v4().to_string();

    let connection = &mut establish_connection();

    let new_role: NewRole = NewRole { id: &id, role_name };

    let created_role: Role = diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result::<Role>(connection)
        .expect("Error saving new role");

    json!(created_role)
}

/*
* Create user details
*/

pub fn create_user(user_details: &UserInputUser) -> Value {
    use workfall_rocket_rs::schema::roles::dsl::*;
    use workfall_rocket_rs::schema::users;

    let connection = &mut establish_connection();

    let appropriate_filter = match &user_details.role_id {
        Some(role_id_value) => role_id_value.to_string(),
        None => "USER".to_string(),
    };

    let mut role: Vec<Role> = roles
    .filter(id.eq(&appropriate_filter)).or_filter(role_name.eq(&appropriate_filter))
    .limit(1)
    .load::<Role>(connection)
    .expect("Error loading role");

    let user_id = uuid::Uuid::new_v4().to_string();

    // Hash password
    let hashed = hash(&user_details.password, DEFAULT_COST).unwrap();

    let new_user: NewUser = NewUser {
        id: &user_id,
        first_name: &user_details.first_name,
        middle_name: &user_details.middle_name,
        last_name: &user_details.last_name,
        email: &user_details.email,
        role_id: &mut role[0].id,
        password: &hashed,
        address:&user_details.address,
        user_name:&user_details.user_name
    };

    let created_user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user");

    json!(created_user)
}

/*
* Update user details
*/
pub fn update_user(user_details: &UserInputUpdateUser) -> Value {
    // use workfall_rocket_rs::schema::users;
    use workfall_rocket_rs::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let existing_user = users
    .filter(email.eq(user_details.email.clone().unwrap()))
    .limit(1)
    .load::<User>(connection)
    .expect("Error fetching user");

    let hashed: String;

    let updated_user_body: NewUser = NewUser {
        id: &existing_user[0].id,
        first_name: &user_details.first_name.clone().unwrap_or(existing_user[0].first_name.clone()),
        middle_name: &user_details.middle_name.clone().unwrap_or(existing_user[0].middle_name.clone().unwrap()),
        last_name: &user_details.last_name.clone().unwrap_or(existing_user[0].last_name.clone()),
        email: &user_details.email.clone().unwrap_or(existing_user[0].email.clone()),
        role_id: &user_details.role_id.clone().unwrap_or(existing_user[0].role_id.clone()),
        password: match &user_details.password {
            Some(new_password) => {
                hashed = hash(new_password, DEFAULT_COST).unwrap();
                &hashed
            },
            None => &existing_user[0].password,
        },
        address:&user_details.address.clone().unwrap_or(existing_user[0].address.clone()),
        user_name:&user_details.user_name.clone().unwrap_or(existing_user[0].user_name.clone()),
    };
    
    let updated_user: User = diesel::update(users.filter(email.eq(user_details.email.clone().unwrap())))
    .set(&updated_user_body)
    .get_result::<User>(connection)
    .expect("Error updating user");

    json!(updated_user)
}
