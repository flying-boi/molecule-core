#![no_std]
use soroban_sdk::{contractimpl, symbol, Address, Env};

pub struct MilestoneEscrow;

#[contractimpl]
impl MilestoneEscrow {
    /// Initialize the escrow with a `payer`, `payee` and `amount`.
    /// Only the `payer` can call `release`.
    pub fn initialize(env: Env, payer: Address, payee: Address, amount: i128) {
        let storage = env.storage();
        storage.set(&symbol!("payer"), &payer);
        storage.set(&symbol!("payee"), &payee);
        storage.set(&symbol!("amount"), &amount);
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

    /// Release the escrow. Only callable by the `payer` recorded at initialization.
    ///
    /// NOTE: This marks the contract as released; actual token/native transfers
    /// should be performed by calling a token contract or by the backend using
    /// a Soroban client. This keeps the example simple and safe for prototyping.
    pub fn release(env: Env) {
        let invoker: Address = env.invoker();
        let payer: Address = env.storage().get_unchecked(&symbol!("payer")).unwrap();

        if invoker != payer {
            panic!("only payer can release");
        }

        env.storage().set(&symbol!("released"), &true);
    }
}
