use super::*;
use syn::Ident;
pub fn impl_payout(struct_name: Ident) -> TokenStream2 {
    let methods = quote! {
        impl #struct_name{
            pub fn get_last_reward(&self, validator_address: &str) -> u32{
                let api = self.api();

                let account: AccountId32;
                match AccountId32::from_ss58check(&validator_address) {
                    Ok(address) => account = address,
                    Err(e) => panic!("Invalid Account id : {:?}", e),
                }

                let active_era = self.active_era();

                let storagekey = api
                    .metadata
                    .storage_map_key("Staking", "Ledger", &account)
                    .unwrap();

                let mut res = StakingLedger {
                    stash: account.clone(),
                    total: 0,
                    active: 0,
                    unlocking: Vec::new(),
                    claimed_rewards: Vec::new(),
                };

                match api.get_storage_by_key_hash(storagekey, None).unwrap() {
                    Some(ledger) => res = ledger,
                    None => (),
                }

                let mut last_reward = 0_u32;
                let is_history_checked_force: bool = false;

                if is_history_checked_force || res.claimed_rewards.len() == 0 {
                    last_reward = api.get_constant("Staking", "HistoryDepth").unwrap();
                    last_reward = active_era.index-last_reward;

                } else {
                    last_reward = res.claimed_rewards.pop().unwrap();
                }
                last_reward
            }

            pub fn payout_call(&self) -> Option<H256>{
                let api = self.set_signer_by_seed();

                let account: AccountId32;
                let account = #struct_name::get_account_id(&self.op_values.address);
                let mut idx =0 ;
                if self.op_values.era == 0{
                    idx = self.active_era().index -1;
                } else{
                    idx = self.op_values.era
                }

                let mut exposure: Exposure<AccountId32, u128> = Exposure {
                    total: 0,
                    own: 0,
                    others: vec![],
                };
                match api
                    .get_storage_double_map("Staking", "ErasStakers", idx, &account, None)
                    .unwrap()
                {
                    Some(exp) => {
                        exposure = exp;
                    }
                    None => (),
                }
                if exposure.total > 0_u128 {
                    let call = api.payout_stakers(idx, account.clone());
                    let result = api.send_extrinsic(call.hex_encode()).unwrap();
                    return result;
                }
                None
            }
        }
    };
    methods
}

pub fn impl_batched_payout(struct_name: Ident) -> TokenStream2 {
    let methods = quote! {
        impl #struct_name{
            pub fn batched_payout(&self) -> Vec<Value> {
                let api = self.api();
                let account: AccountId32;
                match AccountId32::from_ss58check(&self.op_values.address) {
                    Ok(address) => account = address,
                    Err(e) => panic!("Invalid Account id : {:?}", e),
                }
                let pair = sr25519::Pair::from_string(&self.owner_key, None).unwrap();
                let api = api.set_signer(pair.clone());

                let grace_period: GracePeriod = GracePeriod {
                    enabled: false,
                    eras: 0,
                };

                let mut results: Vec<Value> = Vec::new();

                let active_era = self.active_era();
                let mut last_reward = self.get_last_reward(&self.op_values.address);
                let max_batched_transactions = 9;
                let current_active_era = active_era.index;
                let mut num_of_unclaimed_payout = current_active_era - last_reward - 1;
                let mut start = 1;
                let mut num_of_claimed_payouts = 0;

                while num_of_unclaimed_payout > 0 {
                    let mut payout_calls = vec![];
                    let mut tx_limit = num_of_unclaimed_payout;
                    if num_of_unclaimed_payout > max_batched_transactions {
                        tx_limit = max_batched_transactions;
                    }

                    let mut i = start;
                    while i <= tx_limit + start - 1 {
                        let idx = last_reward + i;

                        let is_grace_period_satisfied =
                            !grace_period.enabled || (current_active_era - idx > grace_period.eras);

                        let mut exposure: Exposure<AccountId32, u128> = Exposure {
                            total: 0,
                            own: 0,
                            others: vec![],
                        };

                        match api
                            .get_storage_double_map("Staking", "ErasStakers", idx, &account, None)
                            .unwrap()
                        {
                            Some(exp) => exposure = exp,
                            None => (),
                        }
                        if exposure.total.to_be_bytes() > 0_u128.to_be_bytes() && is_grace_period_satisfied
                        {
                            let call = api.payout_stakers(idx, account.clone());
                            payout_calls.push(call.function);
                        }
                        i += 1;
                        last_reward = last_reward + 1;
                    }
                    let mut current_tx_done = false;
                    let mut payout_calls_len = payout_calls.len();
                    if payout_calls_len > 0 {
                        let batching = api.batch_payout_stakers(payout_calls);
                        let results_hash = api.send_extrinsic(batching.hex_encode()).unwrap();
                        num_of_claimed_payouts += payout_calls_len;

                        let result = serde_json::to_value(results_hash).unwrap();
                        results.push(result);
                    } else {
                        current_tx_done = true;
                    }
                    num_of_unclaimed_payout -= tx_limit;
                    start += tx_limit;
                }
                results
            }

            pub fn get_last_reward(&self, validator_address: &str) -> u32{
                let api = self.api();

                let account: AccountId32;
                match AccountId32::from_ss58check(&validator_address) {
                    Ok(address) => account = address,
                    Err(e) => panic!("Invalid Account id : {:?}", e),
                }

                let active_era = self.active_era();

                let storagekey = api
                    .metadata
                    .storage_map_key("Staking", "Ledger", &account)
                    .unwrap();

                let mut res = StakingLedger {
                    stash: account.clone(),
                    total: 0,
                    active: 0,
                    unlocking: Vec::new(),
                    claimed_rewards: Vec::new(),
                };

                match api.get_storage_by_key_hash(storagekey, None).unwrap() {
                    Some(ledger) => res = ledger,
                    None => (),
                }

                let mut last_reward = 0_u32;
                let is_history_checked_force: bool = false;

                if is_history_checked_force || res.claimed_rewards.len() == 0 {
                    last_reward = api.get_constant("Staking", "HistoryDepth").unwrap();
                    last_reward = active_era.index-last_reward;

                } else {
                    last_reward = res.claimed_rewards.pop().unwrap();
                }
                last_reward
            }
        }
    };
    methods
}
