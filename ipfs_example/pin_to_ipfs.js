// Requires `npm install ipfs-http-client` in this folder
const { create } = require('ipfs-http-client');
const fs = require('fs');

async function main() {
  const client = create({ url: 'https://ipfs.infura.io:5001/api/v0' });
  const data = fs.readFileSync('sample_data.json');
  const result = await client.add(data);
  console.log('IPFS CID:', result.path || result.cid.toString());
}

main().catch(console.error);
