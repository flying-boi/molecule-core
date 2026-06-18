Solidity contracts for the DeSci prototype.

Use Hardhat to compile, test, and deploy the example contracts.
Example contracts:
- `DAO.sol` (simple quorum voting)
- `Escrow.sol` (milestone escrow with release function)

Quick start:

```bash
cd contracts
npm install
npx hardhat compile
npx hardhat test
```

Files:
- `package.json` - dev dependencies and scripts
- `hardhat.config.js` - Hardhat config
- `scripts/deploy.js` - simple deploy script for local testing
- `test/dao-test.js` - minimal unit test for `SimpleDAO`

This project uses Hardhat + Ethers for local development and testing.