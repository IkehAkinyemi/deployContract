use crate::*;

impl DeployContract {
  pub(crate) fn internal_add_account_to_record(&mut self, signer: String, account_id: &AccountId) {

    let mut record_set = self.records.get(&signer).unwrap_or_else(|| {
      // if Drawstring contains no accounts yet we make a new set
      UnorderedSet::new(
        b"D".to_vec(),
      )
    });
    record_set.insert(account_id);
    self.records.insert(&signer, &record_set);
  }
}
