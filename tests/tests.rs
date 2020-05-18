use env_logger::init;
use log::{error, info, warn};
use meilisearch_sdk::{client::*, document::*, errors::Error, indexes::*};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    name: String,
    description: String,
}

// that trait is used by the sdk when the primary key is needed
impl Document for Movie {
    type UIDType = String;

    fn get_uid(&self) -> &Self::UIDType {
        &self.name
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test1() {
    std::panic::catch_unwind(|| init());

    let client = Client::new("http://localhost:7700", "");
    let mut movies = client.get_index("movies").unwrap();

    movies.delete_all_documents().unwrap();
    let status = movies.add_or_replace(vec![Movie{name:String::from("test"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{:?}", status.get_status().unwrap());
}
