#!/bin/sh
set -e

NAME="rwm"

echo "Removing $NAME..."

sudo rm -f "/usr/local/bin/$NAME"

echo "Done!"
