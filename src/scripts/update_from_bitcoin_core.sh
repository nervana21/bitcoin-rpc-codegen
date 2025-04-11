#!/bin/bash

# Script to update the generated code from the latest Bitcoin Core
# This can be run as part of a CI/CD pipeline whenever a new Bitcoin Core version is released

set -e

# Configuration
BITCOIN_CORE_REPO="https://github.com/bitcoin/bitcoin.git"
BITCOIN_CORE_BRANCH="master"
API_JSON_PATH="api.json"
OUTPUT_DIR="generated"

# Clone Bitcoin Core repository
echo "Cloning Bitcoin Core repository..."
git clone --depth=1 --branch=$BITCOIN_CORE_BRANCH $BITCOIN_CORE_REPO bitcoin-core
cd bitcoin-core

# Extract the RPC API documentation
echo "Extracting RPC API documentation..."
./contrib/devtools/gen-manpages.sh

# Copy the generated API documentation
echo "Copying API documentation..."
cp doc/man/rpc/*.json ../$API_JSON_PATH
cd ..

# Run the code generator
echo "Running code generator..."
cargo run --bin bitcoin-rpc-generator -- --api-file $API_JSON_PATH --output-dir $OUTPUT_DIR

# Clean up
echo "Cleaning up..."
rm -rf bitcoin-core

# Commit and push the changes
echo "Committing and pushing changes..."
git add $API_JSON_PATH $OUTPUT_DIR/
git commit -m "Update generated code for Bitcoin Core $(date +%Y-%m-%d)"
git push

echo "Update complete!" 