Soroban (Stellar) contracts for the DeSci prototype.

This repository uses Soroban (Stellar) smart contracts. The Soroban contract
examples and deployment helpers live under `contracts/soroban`.

Quick start (Soroban):

```bash
cd contracts/soroban
# build the contract wasm
soroban build

# set network (e.g. testnet) and deploy
soroban network set testnet
soroban contract deploy target/wasm32-unknown-unknown/release/milestone_escrow_soroban.wasm --network testnet
```

Files to look at:
- `contracts/soroban/src/lib.rs` — MilestoneEscrow contract
- `contracts/soroban/deploy.sh` — simple build+deploy script
- `contracts/soroban/README.md` — usage notes and initialize/release examples

The Solidity/Hardhat artifacts have been removed to make this repository Soroban-only.