Frontend Soroban integration notes

- Use a Soroban-capable JS client (e.g. `@stellar/soroban-client` or similar) to talk to Soroban RPC.
- Typical flow: build transaction (invoke contract), sign with wallet, submit to Soroban RPC.
- For prototypes, call `backend/soroban_client.py` endpoints that wrap `soroban` CLI or RPC calls.

Example JS snippet (conceptual):

```js
// pseudo-code
const client = new SorobanClient(rpcUrl);
const tx = await client.buildInvokeTx(contractId, 'release', []);
await wallet.sign(tx);
await client.submit(tx);
```

Keep UI minimal: create proposals + release buttons that call backend APIs which then call Soroban.
