async function postJSON(url, body) {
  const r = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });
  return r.json();
}

const out = document.getElementById('output');

document.getElementById('build').onclick = async () => {
  out.textContent = 'Building...';
  const res = await postJSON('/soroban/build', {});
  out.textContent = JSON.stringify(res, null, 2);
};

document.getElementById('deploy').onclick = async () => {
  out.textContent = 'Deploying...';
  const res = await postJSON('/soroban/deploy', {});
  out.textContent = JSON.stringify(res, null, 2);
};

document.getElementById('deploy_init').onclick = async () => {
  const payer = prompt('Payer address (submitter):');
  if (!payer) return;
  const payee = prompt('Payee address:');
  if (!payee) return;
  const amount = prompt('Amount (integer):');
  if (!amount) return;
  const token = prompt('Token contract id:');
  if (!token) return;
  out.textContent = 'Deploying and initializing...';
  const res = await postJSON('/soroban/deploy_init', { payer, payee, amount: parseInt(amount, 10), token_contract: token });
  out.textContent = JSON.stringify(res, null, 2);
};

document.getElementById('release').onclick = async () => {
  const id = prompt('Contract ID to invoke release on:');
  if (!id) return;
  out.textContent = 'Invoking release...';
  const res = await postJSON('/soroban/invoke/release', { contract_id: id });
  out.textContent = JSON.stringify(res, null, 2);
};

// DAO UI
document.getElementById('dao_create').onclick = async () => {
  const title = prompt('Proposal title:');
  if (!title) return;
  const description = prompt('Proposal description:');
  if (!description) return;
  const voting_period = prompt('Voting period (seconds):', '3600');
  out.textContent = 'Creating DAO proposal...';
  const res = await postJSON('/dao/create_proposal', { title, description, voting_period: parseInt(voting_period, 10) });
  out.textContent = JSON.stringify(res, null, 2);
};

document.getElementById('dao_vote').onclick = async () => {
  const contract_id = prompt('DAO Contract ID:');
  if (!contract_id) return;
  const proposal_id = prompt('Proposal ID:');
  if (!proposal_id) return;
  const support = confirm('Vote FOR? OK=Yes, Cancel=No');
  out.textContent = 'Submitting vote...';
  const res = await postJSON('/dao/vote', { contract_id, proposal_id: parseInt(proposal_id, 10), support });
  out.textContent = JSON.stringify(res, null, 2);
};
