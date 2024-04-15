#!/bin/bash

# Run cargo build
cargo build

# Check if cargo build was successful
if [ $? -eq 0 ]; then
    # Run the binary file with src and dest parameters
    ./target/debug/cc-unzip-media "$1" "$2"
else
    echo "Cargo build failed. Exiting..."
    exit 1
fi