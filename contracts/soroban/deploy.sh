#!/usr/bin/env bash
set -e

# Build the contract
soroban build

# Set network (example: testnet)
soroban network set testnet

# Deploy the wasm to the selected network
soroban contract deploy target/wasm32-unknown-unknown/release/milestone_escrow_soroban.wasm --network testnet

echo "Deployment command finished. Check output for contract ID."
