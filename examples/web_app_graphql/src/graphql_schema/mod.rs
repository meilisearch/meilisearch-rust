use async_graphql::SimpleObject;
pub mod users;

use users::UsersQuery;

#[derive(Default, SimpleObject)]
pub struct Query {
    users: UsersQuery,
}
