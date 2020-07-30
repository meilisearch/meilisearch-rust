#!/bin/sh

# Checking that cargo is installed
command -v cargo > /dev/null 2>&1
if [ "$?" -ne 0 ]; then
    echo 'You must install cargo to make this script working.'
    echo 'See https://doc.rust-lang.org/cargo/getting-started/installation.html'
    exit
fi

# Installing cargo-readme if it's not installed yet
cargo install cargo-readme

# Generating the README.md file
cargo readme > README.md
