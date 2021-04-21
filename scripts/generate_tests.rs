use yaml_rust::YamlLoader;
use std::{fs::File, io::Write};

// The begining of the generated file
const PRELUDE: &str = r#"
//! This file was generated automatically from the code samples (path: `.code-samples.meilisearch.yaml`).
//! Its goal is to check that all code samples have a valid syntax and run correctly.
//! 
//! It is not possible to edit this file directly.
//! Some parts of this file are also contained in `scripts/generate_tests.rs`.
//! Run `cargo test code_sample -- --test-threads=1` to run all code samples.
//! Please note that markdown code samples are not supported.
#![allow(unused_variables)]
#![allow(unreachable_code)]

use futures_await_test::async_test;
use meilisearch_sdk::{indexes::*, client::*, progress::*, document::*, search::*, settings::*};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[allow(unused_must_use)]
async fn setup_test_index<'a>(client: &'a Client<'a>, name: &'a str) -> Index<'a> {
    let index = client.create_index(name, None).await.unwrap();

    index.add_documents(&[
        Movie {
            id: "25684".to_string(),
            title: "American Ninja 5".to_string(),
            director: "Bobby Jean Leonard".to_string(),
            poster: "https://image.tmdb.org/t/p/w1280/iuAQVI4mvjI83wnirpD8GVNRVuY.jpg".to_string(),
            overview: "When a scientists daughter is kidnapped, American Ninja, attempts to find her, but this time he teams up with a youngster he has trained in the ways of the ninja.".to_string(),
            release_date: 725846400 ,
            rating: 2.0,
            genres: vec![],
        }
    ], None).await.unwrap();
    index.set_attributes_for_faceting(["genres", "director"]).await.unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));
    index
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: String,
    title: String,
    director: String,
    poster: String,
    overview: String,
    release_date: i64,
    rating: f32,
    genres: Vec<String>
}
impl Document for Movie {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType { &self.id }
}

impl Default for Movie {
    fn default() -> Movie {
        Movie {
            id: String::new(),
            title: String::new(),
            director: String::new(),
            poster: String::new(),
            overview: String::new(),
            release_date: 0 ,
            rating: 0.0,
            genres: vec![],
        }
    }
}

"#;

// The structure of each test
const TEST_STRUCTURE: &str = r#"
#[async_test]
async fn code_sample_[NAME]() {
    // Setup
    let client = Client::new("http://localhost:7700", "masterKey");
    [CLEANUP_CODE]
    let movies = setup_test_index(&client, "[NAME]").await;
    [ADDITIONAL_CONFIG]

    // Code sample
    [CODE]

    // Cleanup
    [CLEANUP_CODE]
}
"#;

fn main() {
    // Tell cargo when to rerun this script
    println!("cargo:rerun-if-changed=.code-samples.meilisearch.yaml");
    println!("cargo:rerun-if-changed=build.rs");

    // Read the code samples
    let raw_code_samples = std::fs::read_to_string(".code-samples.meilisearch.yaml").expect("Failed to load code samples");
    let code_samples = YamlLoader::load_from_str(&raw_code_samples).expect("Invalid code samples");
    let code_samples = &code_samples[0].as_hash().expect("Code samples must be a Hash");

    // Initialize the generated code
    let mut generated_file_content = String::new();
    generated_file_content.push_str(PRELUDE);

    // Append each code sample to the generated code
    for (name, code) in code_samples.into_iter() {
        let name = name.as_str().expect("The key of a code sample must be a String");
        let code = code.as_str().expect("The content of a code sample must be a String");

        // Ignore markdown code samples since they are not supported
        if name.ends_with("_md") {
            continue;
        }

        // Format the code
        let code = code.replace("\n", "\n  ");
        let code = code.replace("  ", "    ");

        // Generate the test function
        let mut generated_test = TEST_STRUCTURE.to_string();
        generated_test = generated_test.replace("[NAME]", name);
        generated_test = generated_test.replace("[CODE]", &code);

        // Add custom test workarrounds (some tests would fail without these specific tricks)
        let additionnal_config = match name {
            "get_update_1" => "    let progress = movies.delete_all_documents().await.unwrap();\n",
            "get_dump_status_1" => "    return; // Would fail because this dump does not exist\n",
            _ => "",
        };
        if !additionnal_config.is_empty() {
            let additionnal_config = "\n    // THE FOLLOWING LINES ARE SPECIFIC TO THIS TEST AND CAN ONLY BE EDITED IN `scripts/generate_tests.rs`\n".to_string() + additionnal_config;
            let additionnal_config = additionnal_config + "    // END";
            generated_test = generated_test.replace("[ADDITIONAL_CONFIG]", &additionnal_config);
        } else {
            generated_test = generated_test.replace("[ADDITIONAL_CONFIG]\n", "");
        }

        // Avoid avoidable panics
        generated_test = generated_test.replace("client.get_index(", "client.get_or_create(");

        // Find all indexes created by the test
        let mut created_indexes = Vec::new();
        let mut remaining_code = generated_test.as_str();
        while let Some(idx) = remaining_code.find("create_index(\"") {
            let (_before, after) = remaining_code.split_at(idx+14);
            remaining_code = after;
            let end = after.find('"').unwrap();
            created_indexes.push(after[..end].to_string());
        }
        let mut remaining_code = generated_test.as_str();
        while let Some(idx) = remaining_code.find("get_or_create(\"") {
            let (_before, after) = remaining_code.split_at(idx+15);
            remaining_code = after;
            let end = after.find('"').unwrap();
            created_indexes.push(after[..end].to_string());
        }

        // Add cleanup code to remove created indexes
        let mut cleanup_code = String::new();
        cleanup_code.push_str(&format!("    let _ = client.delete_index(\"{}\").await;\n", name));
        for created_index in created_indexes {
            cleanup_code.push_str(&format!("    let _ = client.delete_index(\"{}\").await;\n", created_index))
        }
        generated_test = generated_test.replace("    [CLEANUP_CODE]\n", &cleanup_code);

        // Append test to file content
        generated_file_content.push_str(&generated_test);
    }

    // Create or update the generated test file
    match std::fs::create_dir("tests") {
        Ok(()) => (),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => panic!("Failed to create `tests` directory: {}", e),
    }
    let mut output_file = File::create("tests/generated_from_code_samples.rs").expect("Failed to open output file");
    output_file.write_all(generated_file_content.as_bytes()).expect("Failed to write to output file");
}
