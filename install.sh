#!/bin/sh
set -e

NAME="rwm"

echo "Building $NAME..."
cargo build --release

echo "Installing $NAME..."
sudo install -Dm755 "target/release/$NAME" "/usr/local/bin/$NAME"

echo "Done!"
