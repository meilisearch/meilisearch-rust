use async_graphql::SimpleObject;
use diesel::{prelude::Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

//Struct that corresponds to our database structure for users table
#[derive(SimpleObject, Deserialize, Serialize, Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
