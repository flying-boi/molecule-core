#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol, symbol_short, Address, Env};
use soroban_sdk::vec::Vec as SorobanVec;
use soroban_sdk::token::TokenClient;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub votes_for: i128,
    pub votes_against: i128,
    pub deadline: u64,
    pub executed: bool,
}

#[contract]
pub struct SimpleDAO;

#[contractimpl]
impl SimpleDAO {
    pub fn create_proposal(env: Env, title: String, description: String, voting_period: u64) -> u64 {
        // load existing proposals vector or create new
        let mut props: SorobanVec<Proposal> = env
            .storage()
            .persistent()
            .get(&symbol_short!("PROPS"))
            .unwrap_or_else(|| SorobanVec::new(&env));

        let mut counter: u64 = env.storage().persistent().get(&symbol_short!("PROP_CT")).unwrap_or(0u64);
        counter = counter + 1;
        env.storage().persistent().set(&symbol_short!("PROP_CT"), &counter);

        let deadline = env.ledger().timestamp() + voting_period;
        let p = Proposal {
            id: counter,
            title,
            description,
            votes_for: 0,
            votes_against: 0,
            deadline,
            executed: false,
        };
        props.push_back(p);
        env.storage().persistent().set(&symbol_short!("PROPS"), &props);
        counter
    }

    pub fn vote(env: Env, proposal_id: u64, support: bool) {
        let invoker: Address = env.invoker();

        // prevent double-vote
        let voted_key = (symbol_short!("VOTED"), proposal_id, invoker.clone());
        let already: Option<bool> = env.storage().persistent().get(&voted_key);
        if already.unwrap_or(false) {
            panic!("already voted");
        }

        let mut props: SorobanVec<Proposal> = env.storage().persistent().get(&symbol_short!("PROPS")).unwrap();
        let len = props.len();
        let mut i = 0u32;
        while i < len {
            let mut pr = props.get(i).unwrap();
            if pr.id == proposal_id {
                if support {
                    pr.votes_for = pr.votes_for + 1;
                } else {
                    pr.votes_against = pr.votes_against + 1;
                }
                props.set(i, pr);
                env.storage().persistent().set(&symbol_short!("PROPS"), &props);
                env.storage().persistent().set(&voted_key, &true);
                return;
            }
            i += 1;
        }
        panic!("proposal not found");
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<Proposal> {
        let props: Option<SorobanVec<Proposal>> = env.storage().persistent().get(&symbol_short!("PROPS"));
        if props.is_none() {
            return None;
        }
        let props = props.unwrap();
        let len = props.len();
        let mut i = 0u32;
        while i < len {
            let pr = props.get(i).unwrap();
            if pr.id == proposal_id {
                return Some(pr);
            }
            i += 1;
        }
        None
    }
}

pub struct MilestoneEscrow;

#[contractimpl]
impl MilestoneEscrow {
    /// Initialize the escrow with a `payer`, `payee`, `amount` and the
    /// `token_contract` address (the token to transfer on release).
    /// Only the `payer` can call `release`.
    pub fn initialize(
        env: Env,
        payer: Address,
        payee: Address,
        amount: i128,
        token_contract: Address,
    ) {
        let storage = env.storage();
        storage.set(&symbol!("payer"), &payer);
        storage.set(&symbol!("payee"), &payee);
        storage.set(&symbol!("amount"), &amount);
        storage.set(&symbol!("token_contract"), &token_contract);
        storage.set(&symbol!("released"), &false);
    }

    pub fn get_payee(env: Env) -> Address {
        env.storage().get_unchecked(&symbol!("payee")).unwrap()
    }

    pub fn get_payer(env: Env) -> Address {
        env.storage().get_unchecked(&symbol!("payer")).unwrap()
    }

    pub fn get_amount(env: Env) -> i128 {
        env.storage().get_unchecked(&symbol!("amount")).unwrap()
    }

    pub fn is_released(env: Env) -> bool {
        env.storage().get_unchecked(&symbol!("released")).unwrap()
    }

    /// Release the escrow: transfers `amount` of `token_contract` from `payer`
    /// to `payee`. Must be invoked by `payer` (the transaction signer) so the
    /// token contract sees valid authorization.
    pub fn release(env: Env) {
        let invoker: Address = env.invoker();
        let payer: Address = env.storage().get_unchecked(&symbol!("payer")).unwrap();

        if invoker != payer {
            panic!("only payer can release");
        }

        let released: bool = env.storage().get_unchecked(&symbol!("released")).unwrap();
        if released {
            panic!("already released");
        }

        let amount: i128 = env.storage().get_unchecked(&symbol!("amount")).unwrap();
        let payee: Address = env.storage().get_unchecked(&symbol!("payee")).unwrap();
        let token_contract: Address = env.storage().get_unchecked(&symbol!("token_contract")).unwrap();

        let token = TokenClient::new(&env, &token_contract);
        // Transfer will require that the payer authorized this invocation
        // (i.e., the payer signed the transaction). The contract enforces
        // that the invoker == payer above.
        token.transfer(&payer, payee, &amount);

        env.storage().set(&symbol!("released"), &true);
    }
}
