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
current_readme="README.md"
generated_readme="README.md_tmp"
cargo readme > "$generated_readme"

# Exiting with the right message
echo ''
diff "$current_readme" "$generated_readme" > /dev/null 2>&1
if [ "$?" = 0 ]; then
    echo "OK"
    rm -f "$generated_readme"
    exit 0
else
    echo "The current README.md is not up-to-date with the template."

    # Displaying the diff if the --diff flag is activated
    if [ "$1" = '--diff' ]; then
        echo 'Diff found:'
        diff "$current_readme" "$generated_readme"
    else
        echo 'To see the diff, run:'
        echo '  $ sh scripts/check-readme.sh --diff'
        echo 'To update the README, run:'
        echo '  $ sh scripts/update-readme.sh'
    fi

    rm -f "$generated_readme"
    exit 1
fi
