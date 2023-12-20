use serde::Deserialize;

//Environment variables required for the app to run
#[derive(Deserialize, Debug, Clone)]
pub struct AppEnvVars {
    pub meilisearch_api_key: String,
    pub meilisearch_host: String,
    pub database_url: String,
    pub migrations_dir_path: String,
}
