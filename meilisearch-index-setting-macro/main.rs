use meilisearch_index_setting_macro::Document;

#[derive(Document)]
struct Movie {
    #[document(primary_key)]
    movie_id: u64,
    #[document(displayed, searchable)]
    title: String,
    #[document(displayed)]
    description: String,
    #[document(filterable, sortable, displayed)]
    release_date: String,
    #[document(filterable, displayed)]
    genres: Vec<String>,
}

impl meilisearch_sdk::documents::Document for Movie {
    fn generate_settings(&self) -> meilisearch_sdk::settings::Settings {
        meilisearch_sdk::settings::Settings::new()
            .with_displayed_attributes([
                "title",
                "description",
                "release_date",
                "genres",
            ])
            .with_sortable_attributes(["release_date"])
            .with_filterable_attributes(["release_date", "genres"])
            .with_searchable_attributes(["title"])
    }
    async fn generate_index(
        client: &meilisearch_sdk::client::Client,
    ) -> std::result::Result<
        meilisearch_sdk::indexes::Index,
        meilisearch_sdk::errors::Error,
    > {
        client
            .create_index("movie", Some("movie_id"))?
            .wait_for_completion(client)?
            .try_make_index(client)
    }
}
fn main() {
    let movie = Movie {
        movie_id: 0,
        title: "".to_string(),
        description: "".to_string(),
        release_date: "".to_string(),
        genres: ::alloc::vec::Vec::new(),
    };
}