use async_graphql::MergedObject;
pub mod get_users;
pub mod search;

use get_users::GetUsers;
use search::SearchUsers;

#[derive(Default, MergedObject)]
pub struct UsersQuery(pub GetUsers, pub SearchUsers);
