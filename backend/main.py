from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel
from typing import List, Optional

from .soroban_client import build_contract, deploy_contract, invoke_release, invoke_contract_function, invoke_contract_function_with_args

app = FastAPI(title="DeSci Labs Prototype API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# serve the simple frontend from /frontend
app.mount("/frontend", StaticFiles(directory="frontend"), name="frontend")


class Proposal(BaseModel):
    id: int
    title: str
    description: str
    goal_eth: float
    votes_for: int = 0
    votes_against: int = 0
    escrow_contract_id: Optional[str] = None
    escrow_deploy_output: Optional[str] = None
    dao_contract_id: Optional[str] = None


proposals: List[Proposal] = []


@app.get("/proposals")
def list_proposals():
    return proposals


@app.post("/proposals")
def create_proposal(p: Proposal, deploy_escrow: Optional[bool] = False, network: Optional[str] = None):
    if deploy_escrow:
        build_res = build_contract()
        if build_res.returncode != 0:
            raise HTTPException(status_code=500, detail={"error": "build failed", "stderr": build_res.stderr})
        deploy_res = deploy_contract(network=network)
        p.escrow_deploy_output = getattr(deploy_res, "stdout", "") or ""
        if deploy_res.returncode == 0:
            import re

            m = re.search(r"([0-9a-fA-F]{40,})", p.escrow_deploy_output)
            if m:
                p.escrow_contract_id = m.group(1)
        proposals.append(p)
        return p
    else:
        proposals.append(p)
        return p


@app.post("/proposals/{proposal_id}/vote")
def vote(proposal_id: int, vote: dict):
    for p in proposals:
        if p.id == proposal_id:
            if vote.get("support"):
                p.votes_for += 1
            else:
                p.votes_against += 1
            return {"status": "ok", "proposal": p}
    raise HTTPException(status_code=404, detail="proposal not found")


@app.post("/dao/create_proposal")
def dao_create_proposal(payload: dict):
    title = payload.get("title")
    description = payload.get("description")
    voting_period = int(payload.get("voting_period", 3600))
    # build & deploy DAO contract
    b = build_contract()
    if b.returncode != 0:
        raise HTTPException(status_code=500, detail={"stage": "build", "stderr": b.stderr})
    d = deploy_contract(network=payload.get("network"))
    if d.returncode != 0:
        raise HTTPException(status_code=500, detail={"stage": "deploy", "stdout": d.stdout, "stderr": d.stderr})
    import re
    out = getattr(d, "stdout", "") or ""
    m = re.search(r"([0-9a-fA-F]{40,128})", out)
    contract_id = m.group(1) if m else None
    if not contract_id:
        raise HTTPException(status_code=500, detail={"stage": "deploy", "stdout": out, "message": "contract id not found in deploy output"})

    # invoke create_proposal
    args = [title, description, str(voting_period)]
    inv = invoke_contract_function_with_args(contract_id, "create_proposal", args=args, network=payload.get("network"))
    if inv.returncode != 0:
        raise HTTPException(status_code=500, detail={"stage": "init", "stdout": inv.stdout, "stderr": inv.stderr})

    return {"contract_id": contract_id, "create_proposal": {"stdout": getattr(inv, "stdout", ""), "stderr": getattr(inv, "stderr", "")}}


@app.post("/dao/vote")
def dao_vote(payload: dict):
    contract_id = payload.get("contract_id")
    proposal_id = int(payload.get("proposal_id"))
    support = bool(payload.get("support"))
    args = [str(proposal_id), "1" if support else "0"]
    inv = invoke_contract_function_with_args(contract_id, "vote", args=args, network=payload.get("network"))
    return {"returncode": inv.returncode, "stdout": getattr(inv, "stdout", ""), "stderr": getattr(inv, "stderr", "")}


@app.post("/soroban/build")
def soroban_build():
    res = build_contract()
    return {"returncode": res.returncode, "stdout": getattr(res, "stdout", ""), "stderr": getattr(res, "stderr", "")}


@app.post("/soroban/deploy")
def soroban_deploy(network: Optional[str] = None):
    res = deploy_contract(network=network)
    return {"returncode": res.returncode, "stdout": getattr(res, "stdout", ""), "stderr": getattr(res, "stderr", "")}


class DeployInitRequest(BaseModel):
    payer: str
    payee: str
    amount: int
    token_contract: str
    network: Optional[str] = None


@app.post("/soroban/deploy_init")
def soroban_deploy_and_init(req: DeployInitRequest):
    # build
    b = build_contract()
    if b.returncode != 0:
        raise HTTPException(status_code=500, detail={"stage": "build", "stderr": b.stderr})

    # deploy
    d = deploy_contract(network=req.network)
    if d.returncode != 0:
        raise HTTPException(status_code=500, detail={"stage": "deploy", "stdout": d.stdout, "stderr": d.stderr})

    # try extract contract id from output
    import re

    out = getattr(d, "stdout", "") or ""
    m = re.search(r"([0-9a-fA-F]{40,128})", out)
    contract_id = m.group(1) if m else None
    if not contract_id:
        raise HTTPException(status_code=500, detail={"stage": "deploy", "stdout": out, "message": "contract id not found in deploy output"})

    # initialize
    args = [req.payer, req.payee, str(req.amount), req.token_contract]
    inv = invoke_contract_function(contract_id, "initialize", args=args, network=req.network)
    return {
        "deploy": {"returncode": d.returncode, "stdout": getattr(d, "stdout", ""), "stderr": getattr(d, "stderr", "")},
        "initialize": {"returncode": inv.returncode, "stdout": getattr(inv, "stdout", ""), "stderr": getattr(inv, "stderr", "")},
        "contract_id": contract_id,
    }


class InvokeRequest(BaseModel):
    contract_id: str


@app.post("/soroban/invoke/release")
def soroban_invoke_release(req: InvokeRequest):
    res = invoke_release(req.contract_id)
    return {"returncode": res.returncode, "stdout": getattr(res, "stdout", ""), "stderr": getattr(res, "stderr", "")}
