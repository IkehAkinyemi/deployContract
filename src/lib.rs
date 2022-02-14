use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};
setup_alloc!();

mod internal;
const INITIAL_DEPOSIT: u128 = 2_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DeployContract {
    records: LookupMap<String, UnorderedSet<AccountId>>,
}

impl Default for DeployContract {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"D".to_vec()),
        }
    }
}

#[near_bindgen]
impl DeployContract {
    pub fn deploy_contract_code(&mut self, account_id: AccountId) -> Promise {
        let attached_deposit = env::attached_deposit();
        assert!(
            INITIAL_DEPOSIT <= attached_deposit,
            "Must attach {} yoctoNEAR to cover storage",
            INITIAL_DEPOSIT
        );
        let refund = attached_deposit - INITIAL_DEPOSIT;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        self.internal_add_account_to_record(env::predecessor_account_id(), &account_id);
        Promise::new(account_id)
            .create_account()
            .transfer(INITIAL_DEPOSIT)
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(include_bytes!("../wasmFile/main.wasm").to_vec())
    }

    pub fn get_record(&self) -> Option<UnorderedSet<AccountId>> {
        self.records.get(&"Drawstring".to_string())
    }
}
