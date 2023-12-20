use async_graphql::{Context, Object, Result};
use diesel_async::RunQueryDsl;

use crate::{models::User, GraphQlData};

#[derive(Default)]
pub struct GetUsers;

#[Object]
impl GetUsers {
    pub async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        use crate::schema::users::dsl::users;

        let GraphQlData { pool, .. } = ctx.data().map_err(|e| {
            log::error!("Failed to get app data: {:?}", e);
            e
        })?;

        let mut connection = pool.get().await?;

        let list_users = users.load::<User>(&mut connection).await?;

        Ok(list_users)
    }
}
