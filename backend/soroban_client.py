"""
Soroban (Stellar) helper wrappers.

Requires the Soroban CLI installed (https://soroban.stellar.org/docs/getting-started/installation)

This module provides simple build/deploy wrappers using the `soroban` CLI as placeholders
for integration. In production you would call RPC endpoints or use an SDK.
"""

from pathlib import Path
import subprocess

CONTRACT_DIR = Path(__file__).resolve().parent.parent / "contracts" / "soroban"
WASM_PATH = CONTRACT_DIR / "target" / "wasm32-unknown-unknown" / "release" / "milestone_escrow_soroban.wasm"


def build_contract():
    """Build the Soroban contract using the `soroban` CLI.

    Returns a `subprocess.CompletedProcess` with captured stdout/stderr.
    """
    cmd = ["soroban", "build"]
    return subprocess.run(
        cmd, check=False, capture_output=True, text=True, cwd=str(CONTRACT_DIR)
    )


def deploy_contract(network: str = None):
    """Deploy the compiled wasm using the `soroban` CLI.
    This is a placeholder — real deployment requires network config and keys.

    If `network` (e.g., `testnet`) is provided, it will be passed to the CLI.
    Returns a `subprocess.CompletedProcess` with captured stdout/stderr.
    """
    if not WASM_PATH.exists():
        raise FileNotFoundError(f"Contract wasm not found at {WASM_PATH}")
    cmd = ["soroban", "contract", "deploy", str(WASM_PATH)]
    if network:
        cmd.extend(["--network", network])
    return subprocess.run(cmd, check=False, capture_output=True, text=True)


def invoke_release(contract_id: str, network: str = None):
    """Invoke the `release` function on a deployed contract (placeholder).

    Returns `subprocess.CompletedProcess` with captured stdout/stderr.
    """
    cmd = ["soroban", "contract", "invoke", "--id", str(contract_id), "--fn", "release"]
    if network:
        cmd.extend(["--network", network])
    return subprocess.run(cmd, check=False, capture_output=True, text=True)
