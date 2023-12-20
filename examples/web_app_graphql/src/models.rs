use async_graphql::SimpleObject;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(SimpleObject, Deserialize, Serialize, Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
