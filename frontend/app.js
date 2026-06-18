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
