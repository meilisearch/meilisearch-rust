#!/bin/bash

cargo install cargo-readme

curent_readme="README.md"
generated_readme="$(cargo readme > README.md)"

diff "$curent_readme" "$generated_readme"
