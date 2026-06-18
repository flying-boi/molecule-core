#![no_std]
use soroban_sdk::{contractimpl, symbol, Address, Env};
use soroban_sdk::token::TokenClient;

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
