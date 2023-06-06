#!/bin/sh

# Checking if current tag matches the package version
current_tag=$(echo $GITHUB_REF | cut -d '/' -f 3 | sed -r 's/^v//')
major=$(echo $current_tag | cut -d '.' -f1)
minor=$(echo $current_tag | cut -d '.' -f2)

file1='Cargo.toml'
file2='README.tpl'
file3='.code-samples.meilisearch.yaml'
file4='README.md'
file5='./meilisearch-index-setting-macro/Cargo.toml'

file_tag1=$(grep '^version = ' $file1 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag_1_1=$(grep '{ path = "meilisearch-index-setting-macro", version =' $file1 | grep -Eo '[0-9]+.[0-9]+.[0-9]+')
file_tag2=$(grep 'meilisearch-sdk = ' $file2 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag3=$(grep 'meilisearch-sdk = ' $file3 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag4=$(grep 'meilisearch-sdk = ' $file4 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
file_tag5=$(grep '^version = ' $file5 | grep -Eo '[0-9]+.[0-9]+.[0-9]+')

if [ "$current_tag" != "$file_tag1" ] ||
  [ "$current_tag" != "$file_tag_1_1" ] ||
  [ "$current_tag" != "$file_tag2" ] ||
  [ "$current_tag" != "$file_tag3" ] ||
  [ "$current_tag" != "$file_tag4" ] ||
  [ "$current_tag" != "$file_tag5" ] \
  ; then
  echo "Error: the current tag does not match the version in package file(s)."
  echo "$file1: found $file_tag1 - expected $current_tag"
  echo "$file1: found $file_tag_1_1 - expected $current_tag"
  echo "$file2: found $file_tag2 - expected $current_tag"
  echo "$file3: found $file_tag3 - expected $current_tag"
  echo "$file4: found $file_tag4 - expected $current_tag"
  echo "$file5: found $file_tag5 - expected $current_tag"
  exit 1
fi

echo 'OK'
exit 0
