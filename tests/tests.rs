use env_logger::init;
use log::{error, info, warn};
use meilisearch_sdk::{client::*, documents::*, errors::Error, indexes::*};

#[test]
fn test() {
    std::panic::catch_unwind(|| init());
}
