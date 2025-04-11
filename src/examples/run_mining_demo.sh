#!/bin/bash

# Run the mining demo example
echo "Running mining demo example..."
cargo run --example mining_demo

# Check if the command was successful
if [ $? -eq 0 ]; then
    echo "Mining demo completed successfully!"
else
    echo "Mining demo failed. Make sure you have a Bitcoin Core v29 node running in regtest mode."
    echo "The node should be accessible at http://127.0.0.1:18443"
    echo "The cookie file should be at /tmp/bitcoin-regtest/.cookie"
fi 