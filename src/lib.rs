use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{near_bindgen, setup_alloc, AccountId, Promise};
setup_alloc!();

mod internal;

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
    pub fn deploy_contract_code(&mut self, account_id: AccountId) -> String {
        self.internal_add_account_to_record(&account_id);
        Promise::new(account_id).deploy_contract(include_bytes!("../wasmFile/main.wasm").to_vec());
        return "Success".to_string();
    }
}
