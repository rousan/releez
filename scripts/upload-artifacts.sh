#!/bin/bash

set -e

export "$(xargs <.env)"

github-release upload \
    --user rousan \
    --repo releez \
    --tag "v$1" \
    --name "releez-v$1-x86_64-apple-darwin.tar.gz" \
    --file "dist/releez-v$1-x86_64-apple-darwin.tar.gz"

github-release upload \
    --user rousan \
    --repo releez \
    --tag "v$1" \
    --name "releez-v$1-x86_64-unknown-linux-gnu.tar.gz" \
    --file "dist/releez-v$1-x86_64-unknown-linux-gnu.tar.gz"

github-release upload \
    --user rousan \
    --repo releez \
    --tag "v$1" \
    --name "releez-v$1-x86_64-pc-windows-gnu.zip" \
    --file "dist/releez-v$1-x86_64-pc-windows-gnu.zip"

