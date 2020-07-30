#!/bin/sh

# Checking that cargo is installed
command -v cargo > /dev/null 2>&1
if [ "$?" -ne 0 ]; then
    echo 'You must install cargo to make this script working.'
    echo 'See https://doc.rust-lang.org/cargo/getting-started/installation.html'
    exit 1
fi

# Installing cargo-readme if it's not installed yet
cargo install cargo-readme

# Comparing the generated README and the current one
curent_readme="README.md"
generated_readme="README.md_tmp"
cargo readme > "$generated_readme"

diff "$curent_readme" "$generated_readme" > /dev/null 2>&1
if [ "$?" = 0 ]; then
    echo "OK"
    rm -f "$generated_readme"
    exit 0
else
    echo "The current README.md is not up-to-date with the template."
    echo "Run: sh scripts/update-readme.sh"
    rm -f "$generated_readme"
    exit 1
fi
