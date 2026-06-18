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

Initialize contract (set payer/payee/token)
-----------------------------------------

After deploying, initialize the contract with the payer, payee, amount and the token contract id. Example (conceptual):

```bash
# replace placeholders with real values returned by deploy
soroban contract invoke --id <CONTRACT_ID> --fn initialize \
	--arg <PAYER_ADDRESS> --arg <PAYEE_ADDRESS> --arg <AMOUNT> --arg <TOKEN_CONTRACT_ID>
```

Then the payer can call `release` to transfer the token to the payee:

```bash
soroban contract invoke --id <CONTRACT_ID> --fn release --auth <PAYER_KEY> --network testnet
```

Note: exact CLI arg formatting may vary by `soroban` version; consult `soroban --help`.
