#!/bin/bash

# if on macos, set the library path
if [ "$(uname)" = "Darwin" ]; then
    echo "Setting library path for macos, please ensure you have installed sdl2 via brew"
    export LIBRARY_PATH="/opt/homebrew/lib"
    export CFLAGS="-I/opt/homebrew/include"
    export LDFLAGS="-L/opt/homebrew/lib"
fi

cargo build --bin server --release
cargo build --bin client --release

# extract the binaries and place in bin folder
mkdir -p bin
cp target/release/server bin/server
cp target/release/client bin/client

echo "Build complete"