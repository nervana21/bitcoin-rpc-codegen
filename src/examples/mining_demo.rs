// SPDX-License-Identifier: CC0-1.0

//! Example demonstrating mining with hidden RPC methods in Bitcoin Core v29.

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use bitcoin_rpc_client::client_sync::v29::Client;
use bitcoin_rpc_client::client_sync::Auth;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Bitcoin Core v29 node
    // Note: This assumes you have a Bitcoin Core v29 node running in regtest mode
    let auth = Auth::CookieFile(PathBuf::from("/tmp/bitcoin-regtest/.cookie"));
    let client = Client::new("http://127.0.0.1:18443", auth)?;

    // Generate a new address to mine to
    let address = client.getnewaddress(None, None)?;
    println!("Mining to address: {}", address);

    // Set mock time to a specific value (regtest only)
    let mock_time = 1234567890;
    client.setmocktime(mock_time)?;
    println!("Set mock time to: {}", mock_time);

    // Generate blocks to the address
    println!("Generating 10 blocks...");
    let block_hashes = client.generatetoaddress(10, address.clone(), None)?;
    println!("Generated {} blocks", block_hashes.len());
    for (i, hash) in block_hashes.iter().enumerate() {
        println!("  Block {}: {}", i + 1, hash);
    }

    // Get the current block count
    let block_count = client.getblockcount()?;
    println!("Current block count: {}", block_count);

    // Wait for a new block
    println!("Waiting for a new block (timeout: 5 seconds)...");
    let block_info = client.waitfornewblock(Some(5))?;
    println!("New block info: {:?}", block_info);

    // Create a transaction
    println!("Creating a raw transaction...");
    let inputs = vec![];
    let outputs = vec![(address.clone(), 0.1)];
    let raw_tx = client.createrawtransaction(inputs, outputs, None, None)?;
    println!("Raw transaction: {}", raw_tx);

    // Fund the transaction
    println!("Funding the transaction...");
    let funded_tx = client.fundrawtransaction(raw_tx, None, None)?;
    println!("Funded transaction: {:?}", funded_tx);

    // Sign the transaction
    println!("Signing the transaction...");
    let signed_tx = client.signrawtransactionwithwallet(funded_tx.hex)?;
    println!("Signed transaction: {:?}", signed_tx);

    // Send the transaction
    println!("Sending the transaction...");
    let txid = client.sendrawtransaction(signed_tx.hex, None)?;
    println!("Transaction sent with ID: {}", txid);

    // Generate a block containing our transaction
    println!("Generating a block containing our transaction...");
    let transactions = vec![signed_tx.hex];
    let block_hash = client.generateblock(address, transactions, Some(true))?;
    println!("Generated block: {:?}", block_hash);

    // Wait for the block to be processed
    println!("Waiting for the block to be processed...");
    thread::sleep(Duration::from_secs(1));

    // Get transaction details
    println!("Getting transaction details...");
    let tx_info = client.gettransaction(txid, None)?;
    println!("Transaction details: {:?}", tx_info);

    println!("Demo completed successfully!");
    Ok(())
}
