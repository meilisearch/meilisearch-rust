use meilisearch_sdk::{Client, Index, Settings};

// we need an async runtime
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client: Client = Client::new("http://localhost:7700", Some("masterKey"));

    // We try to create an index called `movies` with a primary_key of `movie_id`.
    let my_index: Index = client
        .create_index("movies", Some("movie_id"))
        .await
        .expect("Could not join the remote server.")
        // The creation of indexes is asynchronous. But for the sake of the example so we will
        // wait until the update is entirely processed.
        .wait_for_completion(&client, None, None)
        .await
        .expect("Could not join the remote server.")
        // If the creation was successful we can generate an `Index` out of it.
        .try_make_index(&client)
        // This error comes from meilisearch itself.
        .expect("An error happened with the index creation.");

    // And now we can update the settings!
    // You can read more about the available options here: https://www.meilisearch.com/docs/learn/configuration/settings#index-settings
    let settings: Settings = Settings::new()
        .with_searchable_attributes(["name", "title"])
        .with_filterable_attributes(["created_at"]);

    // Updating the settings is also an asynchronous operation.
    let task = my_index
        .set_settings(&settings)
        .await
        .expect("Could not join the remote server.")
        // And here we wait for the operation to execute entirely so we can check any error happened.
        .wait_for_completion(&client, None, None)
        .await
        .expect("Could not join the remote server.");

    // We check if the task failed.
    assert!(
        !task.is_failure(),
        "Could not update the settings. {}",
        task.unwrap_failure().error_message
    );

    // And finally we delete the `Index`.
    my_index
        .delete()
        .await
        .expect("Could not join the remote server.")
        .wait_for_completion(&client, None, None)
        .await
        .expect("Could not join the remote server.");

    // We check if the task failed.
    assert!(
        !task.is_failure(),
        "Could not delete the index. {}",
        task.unwrap_failure().error_message
    );
}
