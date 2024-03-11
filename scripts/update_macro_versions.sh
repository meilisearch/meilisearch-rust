#!/bin/sh
new_version=$(grep '^version = ' Cargo.toml)

# Updates the versions in meilisearch-rust and meilisearch-index-setting-macro of the latter, with the latest meilisearch-rust version.

old_index_macro_version=$(grep '^version = ' ./meilisearch-index-setting-macro/Cargo.toml)
old_macro_in_sdk_version=$(grep '{ path = "meilisearch-index-setting-macro", version =' ./Cargo.toml)

sed -i '' -e "s/^$old_index_macro_version/$new_version/g" './meilisearch-index-setting-macro/Cargo.toml'
sed -i '' -e "s/$old_macro_in_sdk_version/meilisearch-index-setting-macro = { path = \"meilisearch-index-setting-macro\", $new_version }/g" './Cargo.toml'
