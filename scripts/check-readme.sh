#!/bin/bash

# Checking that cargo is installed
if ! command -v cargo &> /dev/null
then
    echo 'You must install cargo to make this script working.'
    echo 'See https://doc.rust-lang.org/cargo/getting-started/installation.html'
    exit
fi

cargo install cargo-readme

curent_readme="README.md"
generated_readme="README.md_tmp"
cargo readme > "$generated_readme"

diff "$curent_readme" "$generated_readme" &> /dev/null
if [[ "$?" -eq 0 ]]; then
    echo "OK"
    rm -f "$generated_readme"
    exit 0
else
    echo "The current README.md is not up-to-date with the template."
    echo "Run: sh scripts/update-readme.sh"
    rm -f "$generated_readme"
    exit 1
fi
