#!/bin/bash
BINARY="aurora"
INSTALL_DIR="/home/abi/etc/bin" # change to your install dir

cargo build --release
cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"