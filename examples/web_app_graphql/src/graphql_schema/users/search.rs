use async_graphql::{Context, Object, Result};
use diesel_async::RunQueryDsl;
use meilisearch_sdk::search::{SearchQuery, SearchResults};

use crate::{models::User, GraphQlData};

#[derive(Default)]
pub struct SearchUsers;

#[Object]
impl SearchUsers {
    async fn search(&self, ctx: &Context<'_>, query_string: String) -> Result<Vec<User>> {
        use crate::schema::users::dsl::users;

        let app_data = ctx.data::<GraphQlData>().map_err(|e| {
            log::error!("Failed to get app data: {:?}", e);
            e
        })?;

        let mut connection = app_data.pool.get().await?;

        let list_users = users.load::<User>(&mut connection).await?;

        match app_data.client.get_index("users").await {
            Ok(index) => {
                index.add_documents(&list_users, Some("id")).await?;
            }
            Err(_) => {
                let task = app_data.client.create_index("users", Some("id")).await?;
                let task = task
                    .wait_for_completion(&app_data.client, None, None)
                    .await?;
                let index = task.try_make_index(&app_data.client).unwrap();

                index.add_documents(&list_users, Some("id")).await?;
            }
        }

        let index = app_data.client.get_index("users").await?;

        let query = SearchQuery::new(&index).with_query(&query_string).build();

        let results: SearchResults<User> = index.execute_query(&query).await?;

        log::error!("{:#?}", results.hits);

        let search_results: Vec<User> = results
            .hits
            .into_iter()
            .map(|hit| User {
                id: hit.result.id,
                email: hit.result.email,
                first_name: hit.result.first_name,
                last_name: hit.result.last_name,
            })
            .collect();

        Ok(search_results)
    }
}
