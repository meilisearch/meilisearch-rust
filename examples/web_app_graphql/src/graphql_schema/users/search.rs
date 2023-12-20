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

        let GraphQlData { pool, client } = ctx.data().map_err(|e| {
            log::error!("Failed to get app data: {:?}", e);
            e
        })?;

        let mut connection = pool.get().await?;

        let list_users = users.load::<User>(&mut connection).await?;

        match client.get_index("users").await {
            //If getting the index is successful, we add documents to it
            Ok(index) => {
                index.add_documents(&list_users, Some("id")).await?;
            }

            //If getting the index fails, we create it and then add documents to the new index
            Err(_) => {
                let task = client.create_index("users", Some("id")).await?;
                let task = task.wait_for_completion(client, None, None).await?;
                let index = task.try_make_index(client).unwrap();

                index.add_documents(&list_users, Some("id")).await?;
            }
        }

        let index = client.get_index("users").await?;

        //We build the query
        let query = SearchQuery::new(&index).with_query(&query_string).build();

        let results: SearchResults<User> = index.execute_query(&query).await?;

        //Tranform the results into a type that implements OutputType
        //Required for return types to implement this trait
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
