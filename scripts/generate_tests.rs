use yaml_rust::YamlLoader;
use std::{fs::File, io::Write};

const PRELUDE: &str = r#"
//! This file was generated automatically from the code samples (path: `.code-samples.meilisearch.yaml`).
//! It is not possible to edit this file directly.
//! Some parts of this file are also contained in `scripts/generate_tests.rs`.
//! Run `cargo test code_sample -- --test-threads=1` to run all code sample tests.
#![allow(unused_variables)]

use futures_await_test::async_test;
use meilisearch_sdk::{indexes::*, client::*, progress::*, document::*, search::*, settings::*};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[allow(unused_must_use)]
async fn setup_test_index<'a>(client: &'a Client<'a>, name: &'a str) -> Index<'a> {
    // try to delete
    client.delete_index(name).await;

    let index = client.create_index(name, None).await.unwrap();

    // We could add data if we wanted to
    /*index.add_documents(&[
        Movie {
            id: "".to_string(),
            title: "".to_string(),
            poster: "".to_string(),
            overview: "".to_string(),
            release_date: 0,
            genres: vec![],
        }
    ], None).await.unwrap();*/

    std::thread::sleep(std::time::Duration::from_millis(500));
    index
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: String,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>
}
impl Document for Movie {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType { &self.id }
}

"#;

const TEST: &str = r#"
#[async_test]
async fn code_sample_[NAME]() {
    let client = Client::new("http://localhost:7700", "masterKey");
    let movies = setup_test_index(&client, "[NAME]").await;
    [ADDITIONAL_CONFIG]

    [CODE]
}
"#;

fn main() {
    println!("cargo:rerun-if-changed=.code-samples.meilisearch.yaml");
    println!("cargo:rerun-if-changed=build.rs");

    // Read the code samples
    let raw_code_samples = std::fs::read_to_string(".code-samples.meilisearch.yaml").expect("Failed to load code samples");
    let code_samples = YamlLoader::load_from_str(&raw_code_samples).expect("Invalid code samples");
    let code_samples = &code_samples[0].as_hash().expect("Code samples must be a Hash");

    // Initialize the code
    let mut generated_file_content = String::new();
    generated_file_content.push_str(PRELUDE);

    // Append each code sample as a test function
    for (name, code) in code_samples.into_iter() {
        let name = name.as_str().expect("The key of a code sample must be a String");
        let code = code.as_str().expect("The content of a code sample must be a String");

        // Formatting
        let code = code.replace("\n", "\n  ");
        let code = code.replace("  ", "    ");

        // Test generation
        let mut generated_test = TEST.to_string();
        generated_test = generated_test.replace("[NAME]", name);
        generated_test = generated_test.replace("[CODE]", &code);

        // Add custom test workarrounds
        let additionnal_config = match name {
            "get_update_1" => "    let progress = movies.delete_all_documents().await.unwrap();\n",
            _ => "",
        };
        if !additionnal_config.is_empty() {
            let additionnal_config = "\n    // THE FOLLOWING LINES ARE SPECIFIC TO THIS TEST AND CAN ONLY BE EDITED IN `scripts/generate_tests.rs`\n".to_string() + additionnal_config;
            let additionnal_config = additionnal_config + "    // END";
            generated_test = generated_test.replace("[ADDITIONAL_CONFIG]", &additionnal_config);
        } else {
            generated_test = generated_test.replace("[ADDITIONAL_CONFIG]\n", "");
        }

        // Append test to file content
        if !name.ends_with("_md") {
            generated_file_content.push_str(&generated_test)
        }
    }

    // Create or update the generated test file
    let mut output_file = File::create("tests/generated_from_code_samples.rs").expect("Failed to open output file");
    output_file.write_all(generated_file_content.as_bytes()).expect("Failed to write to output file");
}


