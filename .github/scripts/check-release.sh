#!/bin/sh

# Checking if current tag matches the package version
current_tag=$(echo $GITHUB_REF | tr -d 'refs/tags/v')
major=$(echo $current_tag | cut -d '.' -f1 )
minor=$(echo $current_tag | cut -d '.' -f2 )
cropped_current_tag="$major.$minor"
file1='Cargo.toml'
file2='src/lib.rs'
file3='.code-samples.meilisearch.yaml'
file4='README.md'

file_tag1=$(grep '^version = ' $file1 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag2=$(grep '//! meilisearch-sdk = ' $file2 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag3=$(grep 'meilisearch-sdk = ' $file3 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag4=$(grep 'meilisearch-sdk = ' $file4 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
if [ "$current_tag" != "$file_tag1" ] || [ "$current_tag" != "$file_tag2" ] || [ "$cropped_current_tag" != "$file_tag3" ] || [ "$current_tag" != "$file_tag4" ]; then
  echo "Error: the current tag does not match the version in package file(s)."
  echo "$file1: found $file_tag1 - expected $current_tag"
  echo "$file2: found $file_tag2 - expected $current_tag"
  echo "$file3: found $file_tag3 - expected $cropped_current_tag"
  echo "$file4: found $file_tag4 - expected $current_tag"
  exit 1
fi

echo 'OK'
exit 0
