use env_logger::init;
use log::{info, warn, error};
use meilisearch_sdk::{client::*, documents::*, errors::Error, indexes::*};

#[test]
fn test() {
    init();
    let client = Client::new("http://localhost:7700", "fzd");

    for index in client.list_all_indexes().unwrap() {
        index.delete().unwrap();
    }
    assert!(client.list_all_indexes().unwrap().is_empty());

    client.create_index("movies", None).unwrap();
    assert_eq!(client.list_all_indexes().unwrap().len(), 1);

    println!("{:?}", client.list_all_indexes())
}

#[test]
fn movie_test() {
    use serde::{Serialize, Deserialize};

    init();

    #[derive(Serialize, Deserialize, Debug)]
    struct Movie {
        id: String,
        description: String,
    }

    let client = Client::new("http://localhost:7700", "fzd");
    client.create_index("movies", None);

    let mut movies = client.get_index("movies").unwrap();
    movies.add_or_replace(vec![Movie{id: String::from("Interstellar"), description: String::from("SpAAAAce")}], None).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let movie: Document<Movie, String> = movies.get_document(String::from("Interstellar")).unwrap();
    println!("{:?}", movie.value);
}