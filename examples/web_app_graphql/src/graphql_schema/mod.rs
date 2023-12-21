use async_graphql::SimpleObject;
pub mod users;

use users::mutation::UsersMut;
use users::query::UsersQuery;

#[derive(Default, SimpleObject)]
pub struct Query {
    users: UsersQuery,
}

#[derive(Default, SimpleObject)]
pub struct Mutation {
    users: UsersMut,
}
