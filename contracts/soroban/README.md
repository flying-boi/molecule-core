Soroban contract (Milestone Escrow)

Prerequisites:
- Rust toolchain with `wasm32-unknown-unknown` target
- `soroban` CLI installed and configured

Build & deploy:

```bash
cd contracts/soroban
soroban build
soroban network set testnet
soroban contract deploy target/wasm32-unknown-unknown/release/milestone_escrow_soroban.wasm --network testnet
```

The `deploy.sh` script automates the above steps. Deployment will print contract information including the contract id; record that id for invocations.
