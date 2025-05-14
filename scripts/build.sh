#!/bin/bash

# if on macos, set the library path
if [ "$(uname)" = "Darwin" ]; then
    export LIBRARY_PATH="/opt/homebrew/lib"
    export CFLAGS="-I/opt/homebrew/include"
    export LDFLAGS="-L/opt/homebrew/lib"
fi

cargo build --bin server
cargo build --bin client

# extract the binaries and place in bin folder
mkdir -p bin
cp target/debug/server bin/server
cp target/debug/client bin/client
