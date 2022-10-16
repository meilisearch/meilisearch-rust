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
fn main() {}
